use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
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

    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos/1")
                .method(Method::DELETE)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
