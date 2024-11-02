//! Database service that provides a SQLx connection pool to the rest of the application

use crate::prelude::*;

use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::Pool;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(url: &str) -> RaveApiResult<Self> {
        let pool =
            Pool::<Postgres>::connect(url)
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