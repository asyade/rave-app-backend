use clap::Parser;
use std::process::abort;
use dotenv::dotenv;
use tracing::{error, info};

use rave_api::prelude::*;

mod log;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        conflicts_with = "database_url",
        help = "Name of the embedded database (will be created if not exists). When using embedded database, the DATABASE_URL environment variable is ignored. The database is not persisted by default but setting the `PG_DATA_DIR` environment variable will make it so."
    )]
    #[cfg(feature = "embedded-database")]
    embeded_database: Option<String>,

    #[arg(
        short,
        long,
        help = "Listen address in format IP:PORT (use environment variable LISTEN_ADDRESS if not set)"
    )]
    address: Option<String>,

    #[arg(
        short,
        long,
        help = "Database URL (use environment variable DATABASE_URL if not set)"
    )]
    database_url: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    log::init();
    let args = Args::parse();

    let database_url;
    // When not using the `embedded_database` feature, just use `args.database_url` as is
    #[cfg(not(feature = "embedded-database"))]
    {
        database_url = args.database_url
    }
    // When using the `embedded-database` feature, handle embedded database creation
    #[cfg(feature = "embedded-database")]
    {
        use rave_embedded_database::prelude::*;

        if let Some(embedded) = args.embeded_database {
            // Handle embedded database creation
            let database_pool = EmbeddedDatabasePool::new().await.unwrap();
            let database = database_pool.create_database(Some(embedded)).await.unwrap();
            database_url = Some(database.connection_string());
        } else {
            // Use regular database_url or env variable if not set
            database_url = args.database_url;
        }
    }

    let options = match RaveApiOptions::try_from_env(args.address, database_url) {
        Ok(options) => options,
        Err(e) => return error!("{}", e),
    };

    match rave_api::serve(options).await {
        Ok(_) => info!("Server stopped gracefully"),
        Err(e) => {
            error!("Server stopped with error: {}", e);
            abort()
        }
    };
}
