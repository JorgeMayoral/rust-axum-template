use std::{error::Error, net::TcpListener};

use axum::{routing::get, Router, Server};

use crate::routes::health_check;

pub struct Application {
    port: u16,
    listener: TcpListener,
    app: Router,
}

impl Application {
    pub async fn build() -> Result<Self, std::io::Error> {
        let port = std::env::var("PORT").unwrap_or_else(|_| "0".to_string());
        let address = format!("{}:{}", "0.0.0.0", port);
        let listener = TcpListener::bind(address)?;

        let app = Router::new().route("/health", get(health_check));

        Ok(Self {
            port: listener.local_addr()?.port(),
            listener,
            app,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Listening on http://0.0.0.0:{}", self.port()); // FIXME: use proper logging

        Server::from_tcp(self.listener.try_clone()?)?
            .serve(self.app.clone().into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}
