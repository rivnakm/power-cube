use sqlx::{Result, SqliteExecutor};

use crate::models::Model;

pub mod migrations;
pub mod solves;

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
