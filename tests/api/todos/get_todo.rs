use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_axum_template::{
    application::Application,
    todos::{inmemory_repository::InMemoryRepository, model::Todo},
};
use tower::ServiceExt;

use crate::helpers::get_default_app;

#[tokio::test]
async fn get_todo_404_test() {
    let app = get_default_app().await.app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/todos/1")
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_todo_test() {
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
                .method(Method::GET)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let todo: Todo = serde_json::from_slice(&body).unwrap();
    assert_eq!(todo, new_todo);
}
