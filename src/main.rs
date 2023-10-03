use rust_axum_template::application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = application::Application::build().await?;
    app.run().await?;

    Ok(())
}
