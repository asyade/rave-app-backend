use dotenv::dotenv;
use rave_api::options::RaveApiOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rave_api=debug,rave_entity=debug,axum=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    rave_api::serve(RaveApiOptions::try_from_env().expect("invalid environ")).await;
}
