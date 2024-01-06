#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod error;
pub mod prelude;

mod graphql;
mod services;

use tokio::net::TcpListener;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQL};
use axum::body::Bytes;
use axum::extract::MatchedPath;
use axum::http::HeaderMap;
use axum::Extension;
use axum::{
    extract::State,
    http::{Request, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use graphql::mutation::Mutation;
use graphql::query::Query;
use graphql::schema::{build_schema, AppSchema};
use prelude::*;
use services::iam::api_user::AnyApiUser;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;

use crate::services::iam::Iam;

pub mod options;

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

#[instrument(skip(options))]
pub async fn serve(options: RaveApiOptions) {
    let db = Database::new()
        .await
        .expect("Failed to initialize database pool");

    let schema: ApiSchema = build_schema(db.clone())
        .await
        .expect("failed to build graphql schema");

    let iam = Iam::init(db, options.auth0.clone())
        .await
        .expect("failed to initialize IAM service");

    let app = Router::new()
        // .layer(Extension(Arc::new(iam)))
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
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
            }),
        )
        .layer(Extension(iam));

    tracing::info!("starting server on {}", options.listen_address);
    let tcp_listener = TcpListener::bind(&options.listen_address)
        .await
        .expect("failed to bind address");
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("failed to start server");
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn graphiql() -> impl IntoResponse {
    tracing::debug!("serving graphiql");
    Html(GraphiQLSource::build().endpoint("/").finish())
}
