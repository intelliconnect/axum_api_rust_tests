use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tower::ServiceExt;

use crate::AppState;

async fn get_app_with_db() -> Router {
    let test_db_url = env::var("TEST_DB").expect("could not get test database url");
    let pg_pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&test_db_url)
        .await
        .expect("could not connect to database");
    let shared_state = Arc::new(AppState { pg_pool });

    crate::app(shared_state)
}

async fn get_app_without_db() -> Router {
    Router::new().route("/", get(|| async move { "axum tests" }))
}

#[tokio::test]
pub async fn test_response_status_ok() {
    let app = get_app_without_db().await;

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // test by status code of the response
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
pub async fn test_response_status_not_found() {
    let app = get_app_without_db().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/not-found")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // test by status code of the response
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
pub async fn test_get_default_user_json() {
    let app = get_app_with_db().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/defaultuser")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let res = json! ({
        "firstname": "test".to_owned(),
        "lastname": "user".to_owned(),
        "age": 20,
    });
    // test by comparing response body
    assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), res);
}
