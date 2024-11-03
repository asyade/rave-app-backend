//! This crate provides a thread-safe wrapper around the `postgresql_embedded` crate.
//! Its enables creating and loading databases easily in testing environments.
//! It can handle multiple databases in parallel allowing to be used in parallel tests environments.
//! It featires database migration by embedding the migrations located at the root migrations folder.

const PG_VERSION: &str = "16.0.0";

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use error::EmbeddedDatabaseResult;
use tracing::info;

pub use postgresql_embedded::*;

pub mod error;
pub mod prelude;

#[derive(Clone)]
pub struct EmbeddedDatabasePool {
    engine: Arc<RwLock<PostgreSQL>>,
}

#[allow(dead_code)]
pub struct EmbeddedDatabase {
    pool: EmbeddedDatabasePool,
    name: String,
    root_user: String,
    root_password: String,
    port: u16,
    host: String,
}

impl EmbeddedDatabasePool {
    pub async fn new() -> EmbeddedDatabaseResult<Self> {
        let mut settings = postgresql_embedded::Settings::default();
        if let Ok(data_dir) = std::env::var("PG_DATA_DIR") {
            settings.data_dir = PathBuf::from(data_dir);
            settings.temporary = false;
            settings.version = VersionReq::parse(PG_VERSION).expect("Invalid PG version");
        }
        let mut db = PostgreSQL::new(settings);
        info!("Setting up database (this might take a while) ...");
        db.setup().await?;
        info!("Starting database ...");
        db.start().await?;
        info!("Database started");
        Ok(Self {
            engine: Arc::new(RwLock::new(db)),
        })
    }

    pub async fn create_database(
        &self,
        name: Option<String>,
    ) -> EmbeddedDatabaseResult<EmbeddedDatabase> {
        let name = name.unwrap_or_else(|| format!("postgres_embedded_{}", uuid::Uuid::new_v4()));
        EmbeddedDatabase::create(self.clone(), &name).await
    }
}

impl EmbeddedDatabase {
    pub async fn create(
        pool: EmbeddedDatabasePool,
        name: &str,
    ) -> EmbeddedDatabaseResult<EmbeddedDatabase> {
        let engine = pool.engine.read().unwrap();
        engine.create_database(name).await?;
        let settings = engine.settings().clone();
        drop(engine);
        Ok(Self {
            name: name.to_string(),
            root_user: settings.username.clone(),
            root_password: settings.password.clone(),
            port: settings.port,
            host: settings.host.clone(),
            pool,
        })
    }

    pub async fn run_migrations(&self) -> EmbeddedDatabaseResult<()> {
        let pool = self.get_pool().await;
        sqlx::migrate!("../../migrations").run(&pool).await?;
        Ok(())
    }

    pub async fn get_pool(&self) -> sqlx::Pool<sqlx::Postgres> {
        sqlx::Pool::connect(&self.connection_string())
            .await
            .unwrap()
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.root_user, self.root_password, self.host, self.port, self.name
        )
    }
}
