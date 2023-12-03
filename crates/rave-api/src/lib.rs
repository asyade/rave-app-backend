#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod error;
pub mod prelude;

mod graphql;
mod services;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use graphql::schema::{build_schema, AppSchema};
use prelude::*;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn serve() {
    dotenvy::dotenv().ok();

    let schema = build_schema()
        .await
        .expect("Failed to build graphql schema !");

    let app = Router::new().route(
        "/api/graphql",
        get(graphiql).post_service(async_graphql_axum::GraphQL::new(schema)),
    );

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
