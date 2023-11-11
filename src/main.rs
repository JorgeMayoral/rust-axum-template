use rust_axum_template::{application, telemetry, todos::inmemory_repository::InMemoryRepository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber =
        telemetry::get_subscriber("rust-axum-template".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let todo_repository = Box::<InMemoryRepository>::default();
    let app = application::Application::build(todo_repository).await?;
    app.run().await?;

    Ok(())
}
