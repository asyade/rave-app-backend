use crate::prelude::*;

use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::Pool;

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> RaveApiResult<Self> {
        let pool =
            Pool::<Postgres>::connect(&std::env::var("DATABASE_URL").map_err(|_| {
                RaveApiError::DatabaseConfig("`DATABASE_URL` must be set".to_string())
            })?)
            .await
            .map_err(|e| {
                RaveApiError::DatabaseConfig(format!("failed to create connections pool: {}", e))
            })?;

        Ok(Database { pool })
    }

    pub async fn acquire(&self) -> RaveApiResult<PoolConnection<Postgres>> {
        Ok(self.pool.acquire().await?)
    }
}

impl AsRef<Pool<Postgres>> for Database {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
}