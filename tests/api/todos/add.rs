use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use rust_axum_template::application::Application;
use tower::ServiceExt;

#[tokio::test]
async fn add_todo_test() {
    let app = Application::build().await.unwrap().app();
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
