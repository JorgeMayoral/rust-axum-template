use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use tower::ServiceExt;

use crate::helpers::get_default_app;

#[tokio::test]
async fn add_todo_test() {
    let app = get_default_app().await.app();
    let json_body = serde_json::to_string(&serde_json::json!({
        "id": 1,
        "title": "test",
        "completed": false
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos")
                .method(Method::POST)
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
