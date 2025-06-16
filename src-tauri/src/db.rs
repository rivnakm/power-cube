use chrono::{DateTime, TimeDelta, Utc};
use sqlx::{Result, Row, SqliteExecutor};

use crate::models::{Model, Solve};

pub trait Repository<TItem>
where
    TItem: Model,
{
    async fn get_all(&self, connection: impl SqliteExecutor<'_>) -> Result<Vec<TItem>>;

    async fn get(&self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<TItem>;

    async fn insert(&mut self, connection: impl SqliteExecutor<'_>, item: &TItem) -> Result<i64>;

    async fn update(
        &mut self,
        connection: impl SqliteExecutor<'_>,
        id: i64,
        item: &TItem,
    ) -> Result<()>;

    async fn delete(&mut self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<()>;
}

#[derive(Default)]
pub struct SolveRepository {}

impl Repository<Solve> for SolveRepository {
    async fn get_all(&self, connection: impl SqliteExecutor<'_>) -> Result<Vec<Solve>> {
        let rows = sqlx::query("SELECT * FROM solves")
            .fetch_all(connection)
            .await?;

        let mut results = Vec::new();

        for row in rows {
            let solve_time = TimeDelta::milliseconds(row.get("solve_time"));
            let timestamp = DateTime::<Utc>::from_timestamp_millis(row.get("timestamp")).unwrap();

            results.push(Solve {
                solve_time,
                timestamp,
            });
        }

        Ok(results)
    }

    async fn get(&self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<Solve> {
        let row = sqlx::query("SELECT * FROM solves WHERE id = ?")
            .bind(id)
            .fetch_one(connection)
            .await?;

        let solve_time = TimeDelta::milliseconds(row.get("solve_time"));
        let timestamp = DateTime::<Utc>::from_timestamp_millis(row.get("timestamp")).unwrap();

        Ok(Solve {
            solve_time,
            timestamp,
        })
    }

    async fn insert(&mut self, connection: impl SqliteExecutor<'_>, item: &Solve) -> Result<i64> {
        let id = sqlx::query("INSERT INTO solves ( solve_time, timestamp ) VALUES ( ?, ? )")
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
        todo!()
    }

    async fn delete(&mut self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeDelta, Utc};
    use sqlx::sqlite::SqlitePoolOptions;

    const SQLITE_IN_MEMORY: &str = ":memory:";

    async fn run_solves_migrations(conn: impl SqliteExecutor<'_>) -> Result<()> {
        sqlx::raw_sql(include_str!("../migrations/20250615_solves.sql"))
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
                solve_time: TimeDelta::minutes(2),
                timestamp: DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis())
                    .unwrap(),
            },
            Solve {
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
}
