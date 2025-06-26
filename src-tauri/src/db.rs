use sqlx::{Result, SqliteExecutor};

use crate::entities::Entity;

pub mod migrations;
pub mod solves;

pub trait Repository<T>
where
    T: Entity,
{
    async fn get_all(&self, connection: impl SqliteExecutor<'_>) -> Result<Vec<T>>;

    async fn get(&self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<T>;

    async fn insert(&mut self, connection: impl SqliteExecutor<'_>, item: &T) -> Result<i64>;

    async fn update(
        &mut self,
        connection: impl SqliteExecutor<'_>,
        id: i64,
        item: &T,
    ) -> Result<()>;

    async fn delete(&mut self, connection: impl SqliteExecutor<'_>, id: i64) -> Result<()>;
}
