use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use hyper::header;
use rust_axum_template::application::Application;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn get_todo_404_test() {
    let app = Application::build().await.unwrap().app();

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
    let app = Application::build().await.unwrap();
    dbg!(&app.port());
    let mut app = app.app();
    let json_body = serde_json::to_string(&serde_json::json!({
        "id": 1,
        "title": "test",
        "completed": false
    }))
    .unwrap();

    let response = app
        .call(
            Request::builder()
                .uri("/todos")
                .method(Method::POST)
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json_body.clone()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

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

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let todo: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(todo.to_string(), json_body);
}
