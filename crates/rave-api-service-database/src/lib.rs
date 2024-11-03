//! Database service that provides a SQLx connection pool to the rest of the application
use error::*;
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::Pool;

pub mod error;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(url: &str) -> DatabaseServiceResult<Self> {
        let pool =
            Pool::<Postgres>::connect(url)
            .await
            .map_err(|e| {
                DatabaseServiceError::DatabaseConfig(format!("failed to create connections pool: {}", e))
            })?;

        Ok(Database { pool })
    }

    pub async fn acquire(&self) -> DatabaseServiceResult<PoolConnection<Postgres>> {
        Ok(self.pool.acquire().await?)
    }
}

impl AsRef<Pool<Postgres>> for Database {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
