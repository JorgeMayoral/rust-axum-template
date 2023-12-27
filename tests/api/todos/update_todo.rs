use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use hyper::header;
use rust_axum_template::{
    application::Application,
    todos::{inmemory_repository::InMemoryRepository, model::Todo},
};
use tower::ServiceExt;

#[tokio::test]
async fn update_todo_test() {
    let mut map: HashMap<u32, Todo> = HashMap::new();
    let new_todo = Todo::new(1, "test".into(), false);
    map.insert(1, new_todo.clone());
    let todo_repo = Box::new(InMemoryRepository::new(map));
    let app = Application::build(todo_repo).await.unwrap();
    let app = app.app();
    let json_body = serde_json::to_string(&serde_json::json!({
        "id": 1,
        "title": "test-modified",
        "completed": true
    }))
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos/1")
                .method(Method::PUT)
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let updated_todo: Todo = serde_json::from_slice(&body).unwrap();
    let expected_todo = Todo::new(1, "test-modified".into(), true);
    assert_eq!(updated_todo, expected_todo);
}
