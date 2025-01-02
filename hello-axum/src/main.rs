use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Error;
use axum::Router;
use tokio::net::TcpListener;
use web::WebApp;
mod user;
mod web;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", addr);
    let web_app = Arc::new(WebApp::new("sqlite:web.db").await?);
    let app = Router::new().with_state(web_app.clone());
    axum::serve(tcp_listener, app).await.unwrap();
    Ok(())
}
