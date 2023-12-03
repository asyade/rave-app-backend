#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod error;
pub mod prelude;

mod graphql;
mod services;

use std::time::Duration;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::HeaderMap;
use axum::{
    extract::State,
    http::{Request, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use graphql::schema::{build_schema, AppSchema};
use prelude::*;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};

pub async fn serve() {
    let schema = build_schema()
        .await
        .expect("Failed to build graphql schema !");

    let app = Router::new()
        .route(
            "/",
            get(graphiql).post_service(async_graphql_axum::GraphQL::new(schema)),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // created above.
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {},
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {},
                ),
        );

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn graphiql() -> impl IntoResponse {
    tracing::debug!("serving graphiql");
    Html(GraphiQLSource::build().endpoint("/").finish())
}
