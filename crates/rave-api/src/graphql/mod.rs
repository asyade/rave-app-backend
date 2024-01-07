/// Custom re-implementations of async_graphql_axum's GraphQL service
pub mod mutation;
pub mod query;
pub mod schema;

use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Duration,
};

use async_graphql::{
    futures_util::{future::BoxFuture, StreamExt},
    BatchResponse, ServerError,
};
use async_graphql::{
    http::{create_multipart_mixed_stream, is_accept_multipart_mixed},
    Executor,
};
use axum::{
    body::{Body, HttpBody},
    extract::FromRequest,
    http::{Request as HttpRequest, Response as HttpResponse},
    response::IntoResponse,
    BoxError,
};
use bytes::Bytes;
use rave_api_iam::{api_user::AnyApiUser, Iam};
use tower_http::cors::Any;
use tower_service::Service;

use async_graphql_axum::{GraphQLBatchRequest, GraphQLRequest, GraphQLResponse};

/// A GraphQL service.
#[derive(Clone)]
pub struct GraphQL<E> {
    executor: E,
    iam: Iam,
}

impl<E> GraphQL<E> {
    /// Create a GraphQL handler.
    pub fn new(executor: E, iam: Iam) -> Self {
        Self { executor, iam }
    }
}

impl<B, E> Service<HttpRequest<B>> for GraphQL<E>
where
    B: HttpBody<Data = Bytes> + Send + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    E: Executor,
{
    type Response = HttpResponse<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HttpRequest<B>) -> Self::Future {
        let executor = self.executor.clone();
        let req = req.map(Body::new);
        let iam = self.iam.clone();
        Box::pin(async move {
            let authorixation_token = req
                .headers()
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .map(|value| value.trim_start_matches("Bearer "));

            let user = if let Some(authorixation_token) = authorixation_token {
                match iam.api_user_from_token(authorixation_token).await {
                    Ok(user) => AnyApiUser::Identified(user),
                    Err(err) => {
                        return Ok(
                            build_error_response("Failed to get user from token").into_response()
                        );
                    }
                }
            } else {
                AnyApiUser::Guest
            };

            let is_accept_multipart_mixed = req
                .headers()
                .get("accept")
                .and_then(|value| value.to_str().ok())
                .map(is_accept_multipart_mixed)
                .unwrap_or_default();

            if is_accept_multipart_mixed {
                let req =
                    match GraphQLRequest::<rejection::GraphQLRejection>::from_request(req, &())
                        .await
                    {
                        Ok(req) => req,
                        Err(err) => return Ok(err.into_response()),
                    };
                    let stream = executor.execute_stream(req.0.data(user), None);
                let body = Body::from_stream(
                    create_multipart_mixed_stream(
                        stream,
                        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(
                            Duration::from_secs(30),
                        ))
                        .map(|_| ()),
                    )
                    .map(Ok::<_, std::io::Error>),
                );
                Ok(HttpResponse::builder()
                    .header("content-type", "multipart/mixed; boundary=graphql")
                    .body(body)
                    .expect("BUG: invalid response"))
            } else {
                let req = match GraphQLBatchRequest::<rejection::GraphQLRejection>::from_request(
                    req,
                    &(),
                )
                .await
                {
                    Ok(req) => req,
                    Err(err) => return Ok(err.into_response()),
                };
                Ok(GraphQLResponse(executor.execute_batch(req.0.data(user)).await).into_response())
            }
        })
    }
}

fn build_error_response(message: &str) -> async_graphql_axum::GraphQLResponse {
    async_graphql_axum::GraphQLResponse(BatchResponse::Single(
        async_graphql::Response::from_errors(vec![ServerError::new(message, None)]),
    ))
}

/// Rejection response types.
pub mod rejection {
    use async_graphql::ParseRequestError;
    use axum::{
        body::Body,
        http,
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    /// Rejection used for [`GraphQLRequest`](GraphQLRequest).
    pub struct GraphQLRejection(pub ParseRequestError);

    impl IntoResponse for GraphQLRejection {
        fn into_response(self) -> Response {
            match self.0 {
                ParseRequestError::PayloadTooLarge => http::Response::builder()
                    .status(StatusCode::PAYLOAD_TOO_LARGE)
                    .body(Body::empty())
                    .unwrap(),
                bad_request => http::Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(format!("{:?}", bad_request)))
                    .unwrap(),
            }
        }
    }

    impl From<ParseRequestError> for GraphQLRejection {
        fn from(err: ParseRequestError) -> Self {
            GraphQLRejection(err)
        }
    }
}
