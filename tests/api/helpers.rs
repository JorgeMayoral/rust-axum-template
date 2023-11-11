use rust_axum_template::{
    application::Application, todos::inmemory_repository::InMemoryRepository,
};

pub async fn get_default_app() -> Application {
    let todo_repository = Box::<InMemoryRepository>::default();
    Application::build(todo_repository).await.unwrap()
}
