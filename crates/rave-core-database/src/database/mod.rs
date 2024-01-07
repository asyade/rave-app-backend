use crate::prelude::*;

use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::Pool;

pub type PgSid = i32;
pub use sqlx::types::Uuid as PgUuid;

mod cursor;
pub use cursor::*;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

pub struct DatabaseConnection {
    pub connection: PoolConnection<Postgres>,
}

impl Database {
    pub async fn new(database_url: impl AsRef<str>) -> Result<Self, CoreDatabaseError> {
        let pool = Pool::<Postgres>::connect(database_url.as_ref())
            .await?;

        Ok(Database { pool })
    }

    pub async fn acquire(&self) -> Result<PoolConnection<Postgres>, CoreDatabaseError> {
        Ok(self.pool.acquire().await?)
    }
}

impl AsRef<Pool<Postgres>> for Database {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl AsRef<PoolConnection<Postgres>> for DatabaseConnection {
    fn as_ref(&self) -> &PoolConnection<Postgres> {
        &self.connection
    }
}

impl AsMut<PoolConnection<Postgres>> for DatabaseConnection {
    fn as_mut(&mut self) -> &mut PoolConnection<Postgres> {
        &mut self.connection
    }
}