#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod error;
pub mod prelude;

use prelude::*;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
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
use rave_api_graphql::mutation::Mutation;
use rave_api_graphql::query::Query;
use rave_api_graphql::schema::{build_schema, AppSchema};

use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;

pub mod options;

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

/// The API state contains the GraphQL schema and may contains other state in the future
/// > It is important to keep in mind that the state must be optional or shared using a cache system to keep the ability to scale horizontally the api
#[derive(Clone)]
pub struct ApiState {
    pub schema: ApiSchema,
}

#[instrument(skip(options))]
pub async fn serve(options: RaveApiOptions) -> RaveApiResult<()> {
    // Initialize the database pool
    let db = Database::new(&options.database_url).await?;

    // Initialize the GraphQL schema and wrap it in the `ApiState``
    let state = ApiState {
        schema: build_schema(db.clone()).await?,
    };

    // Initialize the authentication layer
    let authentication_layer = Extension(Iam::init(db, options.auth0.clone()).await?);

    // Initialize the tracing layer
    let tracing_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
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
    });

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(tracing_layer)
        .layer(authentication_layer)
        .with_state(state);

    axum::Server::bind(&options.listen_address)
        .serve(app.into_make_service())
        .await
        .map_err(|e| RaveApiError::Http(e.to_string()))?;
    Ok(())
}

/// The GraphQL handler is the entry point for all GraphQL requests that came from the webserver
/// 
/// # Arguments
/// * `state` - The global state of the app
/// * `user` - The user that made the request (extracted from the JWT)
/// * `req` - The GraphQL request
#[instrument(skip(req, user, schema), fields(api_user = %user))]
async fn graphql_handler(
    State(ApiState { schema, .. }): State<ApiState>,
    user: AnyApiUser,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(user)).await.into()
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn graphiql() -> impl IntoResponse {
    tracing::debug!("serving graphiql");
    Html(GraphiQLSource::build().endpoint("/").finish())
}
