use axum::{routing::get, Router};
use tokio::net::TcpListener;
use common::config::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get config file
    let config = Config::new()?;

    // Register handlers
    let router = Router::new().route("/", get(|| async {"Hello world"}));

    // Create listener
    let listener = TcpListener::bind(format!("0.0.0.0:{}", &config.app_port)).await?;

    // Run server
    axum::serve(listener, router).await?;

    Ok(())
}
