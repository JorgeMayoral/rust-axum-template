use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rust_axum_template::application::Application;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_test() {
    let app = Application::build().await.unwrap().app();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
