use std::str::FromStr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub enum LogLevel {
    Debug,
    Normal,
    Quiet,
}

pub fn init() {
    tracing_subscriber::registry()
        .with(log_level().into_env_filter())
        .with(tracing_subscriber::fmt::layer().pretty())
        .init()
}

fn log_level() -> LogLevel {
    match std::env::var("RAVE_LOG_LEVEL").as_deref() {
        Ok("debug") => LogLevel::Debug,
        Ok("normal") => LogLevel::Normal,
        Ok("quiet") => LogLevel::Quiet,
        _ => LogLevel::Normal,
    }
}

impl LogLevel {
    fn into_env_filter(self) -> tracing_subscriber::EnvFilter {
        match self {
            LogLevel::Debug => tracing_subscriber::EnvFilter::from_str(
                "rave_api=trace,rave_entity=trace,axum=trace,axum::rejection=trace,hyper=error,axum_jwks=trace",
            ).unwrap(),
            LogLevel::Normal => tracing_subscriber::EnvFilter::from_str(
                "rave_api=debug,rave_entity=debug,axum=info,axum::rejection=error,hyper=error,axum_jwks=warn",
            )
            .unwrap(),
            LogLevel::Quiet => tracing_subscriber::EnvFilter::from_str("warn").unwrap(),
        }
    }
}
