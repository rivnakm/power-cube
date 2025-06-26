use chrono::{DateTime, TimeDelta, Utc};
use sqlx::{Result, Row, SqliteExecutor};

use crate::entities::Solve;

use super::Repository;

#[derive(Default)]
pub struct SolveRepository {}

impl SolveRepository {
    /// Get the average time of the most recent n solves
    pub async fn get_avg_of_n(
        &self,
        connection: impl SqliteExecutor<'_>,
        n: u32,
    ) -> Result<TimeDelta> {
        let rows = sqlx::query(
            r#"
            SELECT solve_time
            FROM solves
            ORDER BY timestamp DESC
            LIMIT $1
        "#,
        )
        .bind(n)
        .fetch_all(connection)
        .await?;

        let times = rows
            .into_iter()
            .map(|r| r.get::<i64, _>("solve_time"))
            .collect::<Vec<i64>>();

        if times.is_empty() {
            return Ok(TimeDelta::zero());
        }
        let avg = times.iter().sum::<i64>() / times.len() as i64;

        Ok(TimeDelta::milliseconds(avg))
    }

    pub async fn get_best_time(
        &self,
        connection: impl SqliteExecutor<'_>,
    ) -> Result<Option<TimeDelta>> {
        let row = sqlx::query(
            r#"
            SELECT solve_time
            FROM solves
            ORDER BY solve_time ASC
            LIMIT 1
        "#,
        )
        .fetch_optional(connection)
        .await?;

        Ok(row.map(|row| TimeDelta::milliseconds(row.get("solve_time"))))
    }

    /// Get the average of the best n solves
    pub async fn get_best_avg_of_n(
        &self,
        connection: impl SqliteExecutor<'_>,
        n: u32,
    ) -> Result<TimeDelta> {
        let rows = sqlx::query(
            r#"
            SELECT solve_time
            FROM solves
            ORDER BY solve_time ASC
            LIMIT $1
        "#,
        )
        .bind(n)
        .fetch_all(connection)
        .await?;

        let times = rows
            .into_iter()
            .map(|r| r.get::<i64, _>("solve_time"))
            .collect::<Vec<i64>>();
        if times.is_empty() {
            return Ok(TimeDelta::zero());
        }
        let avg = times.iter().sum::<i64>() / times.len() as i64;

        Ok(TimeDelta::milliseconds(avg))
    }
}

impl Repository<Solve> for SolveRepository {
    async fn get_all(&self, connection: impl SqliteExecutor<'_>) -> Result<Vec<Solve>> {
        let rows = sqlx::query("SELECT * FROM solves")
            .fetch_all(connection)
            .await?;

        let mut results = Vec::new();

        for row in rows {
            let id = row.get("id");
            let solve_time = TimeDelta::milliseconds(row.get("solve_time"));
            let timestamp = DateTime::<Utc>::from_timestamp_millis(row.get("timestamp")).unwrap();

            results.push(Solve {
                id,
                solve_time,
                timestamp,
            });
        }

        Ok(results)
    }

    async fn get(&self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<Solve> {
        let row = sqlx::query("SELECT * FROM solves WHERE id = $1")
            .bind(id)
            .fetch_one(connection)
            .await?;

        let solve_time = TimeDelta::milliseconds(row.get("solve_time"));
        let timestamp = DateTime::<Utc>::from_timestamp_millis(row.get("timestamp")).unwrap();

        Ok(Solve {
            id,
            solve_time,
            timestamp,
        })
    }

    async fn insert(&mut self, connection: impl SqliteExecutor<'_>, item: &Solve) -> Result<i64> {
        let id = sqlx::query("INSERT INTO solves ( solve_time, timestamp ) VALUES ( $1, $2 )")
            .bind(item.solve_time.num_milliseconds())
            .bind(item.timestamp.timestamp_millis())
            .execute(connection)
            .await?
            .last_insert_rowid();

        Ok(id)
    }

    async fn update(
        &mut self,
        connection: impl SqliteExecutor<'_>,
        id: i64,
        item: &Solve,
    ) -> Result<()> {
        _ = sqlx::query("UPDATE solves SET solve_time = $1, timestamp = $2 WHERE id = $3")
            .bind(item.solve_time.num_milliseconds())
            .bind(item.timestamp.timestamp_millis())
            .bind(id)
            .execute(connection)
            .await?;

        Ok(())
    }

    async fn delete(&mut self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<()> {
        _ = sqlx::query("DELETE FROM solves WHERE id = $1")
            .bind(id)
            .execute(connection)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeDelta, Utc};
    use sqlx::sqlite::SqlitePoolOptions;

    const SQLITE_IN_MEMORY: &str = ":memory:";

    async fn run_solves_migrations(conn: impl SqliteExecutor<'_>) -> Result<()> {
        sqlx::raw_sql(include_str!("../../migrations/20250615_solves.sql"))
            .execute(conn)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn insert_solve() {
        let pool = SqlitePoolOptions::new()
            .connect(SQLITE_IN_MEMORY)
            .await
            .unwrap();

        let mut conn = pool.acquire().await.unwrap();
        run_solves_migrations(&mut *conn).await.unwrap();

        let solve = Solve {
            id: 1,
            solve_time: TimeDelta::minutes(2),
            timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                .unwrap(),
        };

        let mut repo = SolveRepository::default();
        let id = repo.insert(&mut *conn, &solve).await.unwrap();

        let actual = repo.get(&mut *conn, id).await.unwrap();

        assert_eq!(solve, actual);
    }

    #[tokio::test]
    async fn get_all_solves() {
        let pool = SqlitePoolOptions::new()
            .connect(SQLITE_IN_MEMORY)
            .await
            .unwrap();

        let mut conn = pool.acquire().await.unwrap();
        run_solves_migrations(&mut *conn).await.unwrap();

        let solves = [
            Solve {
                id: 1,
                solve_time: TimeDelta::minutes(2),
                timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                    .unwrap(),
            },
            Solve {
                id: 2,
                solve_time: TimeDelta::minutes(1),
                timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                    .unwrap(),
            },
        ];

        let mut repo = SolveRepository::default();

        for solve in solves.iter() {
            _ = repo.insert(&mut *conn, solve).await.unwrap();
        }

        let actual = repo.get_all(&mut *conn).await.unwrap();

        assert_eq!(solves.len(), actual.len());

        for i in 0..solves.len() {
            assert_eq!(solves[i], actual[i]);
        }
    }

    #[tokio::test]
    async fn delete_solve() {
        let pool = SqlitePoolOptions::new()
            .connect(SQLITE_IN_MEMORY)
            .await
            .unwrap();

        let mut conn = pool.acquire().await.unwrap();
        run_solves_migrations(&mut *conn).await.unwrap();

        let solve = Solve {
            id: 1,
            solve_time: TimeDelta::minutes(2),
            timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                .unwrap(),
        };

        let mut repo = SolveRepository::default();
        let id = repo.insert(&mut *conn, &solve).await.unwrap();

        repo.delete(&mut *conn, id).await.unwrap();

        let get_result = repo.get(&mut *conn, id).await;

        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn update_solve() {
        let pool = SqlitePoolOptions::new()
            .connect(SQLITE_IN_MEMORY)
            .await
            .unwrap();

        let mut conn = pool.acquire().await.unwrap();
        run_solves_migrations(&mut *conn).await.unwrap();

        let solve = Solve {
            id: 1,
            solve_time: TimeDelta::minutes(2),
            timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                .unwrap(),
        };

        let mut repo = SolveRepository::default();
        let id = repo.insert(&mut *conn, &solve).await.unwrap();

        let new_solve = Solve {
            id: 1,
            solve_time: TimeDelta::minutes(1),
            timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                .unwrap(),
        };

        repo.update(&mut *conn, id, &new_solve).await.unwrap();

        let actual = repo.get(&mut *conn, id).await.unwrap();

        assert_eq!(new_solve, actual);
    }

    #[tokio::test]
    async fn avg_of_3() {
        let pool = SqlitePoolOptions::new()
            .connect(SQLITE_IN_MEMORY)
            .await
            .unwrap();

        let mut conn = pool.acquire().await.unwrap();
        run_solves_migrations(&mut *conn).await.unwrap();

        let solves = [
            Solve {
                id: 1,
                solve_time: TimeDelta::minutes(3),
                timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                    .unwrap(),
            },
            Solve {
                id: 2,
                solve_time: TimeDelta::minutes(1),
                timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                    .unwrap(),
            },
            Solve {
                id: 3,
                solve_time: TimeDelta::minutes(2),
                timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                    .unwrap(),
            },
        ];

        let mut repo = SolveRepository::default();

        for solve in solves.iter() {
            _ = repo.insert(&mut *conn, solve).await.unwrap();
        }

        let ao3 = repo.get_avg_of_n(&mut *conn, 3).await.unwrap();

        assert_eq!(ao3.num_minutes(), 2);
    }
}
