use crate::prelude::*;

use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::Pool;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: impl AsRef<str>) -> Result<Self, sqlx::Error> {
        let pool = Pool::<Postgres>::connect(database_url.as_ref())
            .await?;

        Ok(Database { pool })
    }

    pub async fn acquire(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        Ok(self.pool.acquire().await?)
    }
}

impl AsRef<Pool<Postgres>> for Database {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
