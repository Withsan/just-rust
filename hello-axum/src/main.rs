use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Error;
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnRequest, TraceLayer};
use tracing::Level;
use web::WebApp;
mod web;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 9999));
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    let web_app = Arc::new(WebApp::new("sqlite:web.db").await?);
    let app = Router::new()
        .merge(web::user::router())
        .route_layer(middleware::from_fn(web::auth::authentication))
        .merge(web::auth::router())
        .with_state(web_app.clone())
        .layer(TraceLayer::new_for_http().on_request(DefaultOnRequest::new().level(Level::INFO)));
    tracing::info!("server is running on {:?}", addr);
    axum::serve(tcp_listener, app).await.unwrap();
    Ok(())
}
