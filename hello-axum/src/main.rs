use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root)).route("/foo", get(bar));
    axum::Server::bind(&"127.0.0.1:9999".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn root() -> String {
    "fuck root".to_string()
}
async fn bar() -> String {
    "fuck foo".to_string()
}
