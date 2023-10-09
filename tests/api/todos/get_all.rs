use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use rust_axum_template::application::Application;
use tower::ServiceExt;

#[tokio::test]
async fn get_all_todos_test() {
    let app = Application::build().await.unwrap().app();

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

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let todos: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(todos.len(), 0);
}
