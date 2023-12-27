use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use crate::helpers::get_default_app;

#[tokio::test]
async fn get_all_todos_test() {
    let app = get_default_app().await.app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let todos: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(todos.len(), 0);
}
