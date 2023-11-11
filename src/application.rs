use std::{error::Error, net::TcpListener};

use axum::{routing::get, Router, Server};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::health::controllers::health_check;
use crate::todos::repository::TodoRepository;
use crate::todos::router::todos_router;

pub struct Application {
    port: u16,
    listener: TcpListener,
    app: Router,
}

impl Application {
    pub async fn build(todo_repository: TodoRepository) -> Result<Self, std::io::Error> {
        let port = std::env::var("PORT").unwrap_or_else(|_| "0".to_string());
        let address = format!("{}:{}", "0.0.0.0", port);
        let listener = TcpListener::bind(address)?;

        let trace_layer = TraceLayer::new_for_http();

        let app = Router::new()
            .route("/health", get(health_check))
            .nest("/todos", todos_router(todo_repository))
            .layer(trace_layer);

        Ok(Self {
            port: listener.local_addr()?.port(),
            listener,
            app,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn app(self) -> Router {
        self.app
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        info!("Listening on http://0.0.0.0:{}", self.port());

        Server::from_tcp(self.listener.try_clone()?)?
            .serve(self.app.clone().into_make_service())
            .await
            .expect("Falied to run server");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::todos::inmemory_repository::InMemoryRepository;

    #[tokio::test]
    async fn test_build() {
        std::env::set_var("PORT", "8080");
        let todo_repository = Box::<InMemoryRepository>::default();
        let app = super::Application::build(todo_repository).await.unwrap();
        assert_eq!(app.port(), 8080);
    }
}
