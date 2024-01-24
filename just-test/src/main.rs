mod my;
use tracing::Level;
#[tokio::main]
async fn main() {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    let mut twelve_bytes = vec![0; 12];
    let num = 1i32;
    let num_bytes = num.to_be_bytes();
    twelve_bytes[8..].copy_from_slice(&num_bytes);
    for one_byte in twelve_bytes {
        tracing::info!(one_byte);
    }
}
