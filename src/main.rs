use rust_axum_template::{application, telemetry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber =
        telemetry::get_subscriber("rust-axum-template".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let app = application::Application::build().await?;
    app.run().await?;

    Ok(())
}
