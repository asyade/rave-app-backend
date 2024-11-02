#![feature(exitcode_exit_method)]

use std::process::ExitCode;

use dotenv::dotenv;
use tracing::{error, info};
use rave_api::prelude::*;
mod log;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        conflicts_with = "database_url",
        help = "Name of the embedded database (will be created if not exists). When using embedded database, the DATABASE_URL environment variable is ignored.")]
    #[cfg(feature = "embedded-database")]
    embeded_database: Option<String>,

    #[arg(short, long, help = "Listen address in format IP:PORT (use environment variable LISTEN_ADDRESS if not set)")]
    address: Option<String>,

    #[arg(short, long, help = "Database URL (use environment variable DATABASE_URL if not set)")]
    database_url: Option<String>,
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    log::init();
    
    let args = Args::parse();
    let database_url;

    #[cfg(feature="embedded-database")] {
        use rave_embedded_database::prelude::*;
        if let Some(embedded) = args.embeded_database {
            let database_pool = EmbeddedDatabasePool::new().await.unwrap();
            let database = database_pool.create_database(Some(embedded)).await.unwrap();
            database_url = Some(database.connection_string());
        } else {
            database_url = args.database_url;
        }
    }

    #[cfg(not(feature="embedded-database"))] {
        database_url = args.database_url
    }

    let options = match RaveApiOptions::try_from_env(args.address, database_url) {
        Ok(options) => options,
        Err(e) => return error!("{}", e),
    };

    match rave_api::serve(options).await {
        Ok(_) => info!("Server stopped gracefully"),
        Err(e) => {
            error!("Server stopped with error: {}", e);

            // Exit the process with platform specific failure exit status
            ExitCode::FAILURE.exit_process()
        }
    };
}
