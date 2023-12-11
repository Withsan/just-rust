use etcd_client::Error;
use tracing::Level;
mod key_value;
mod lease;
mod lock;
mod watch;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    tracing::info!("hello tracing");
    // key_value::key_value().await?;
    // lease::lease().await?;
    // watch::watch().await?;
    lock::lock().await?;
    Ok(())
}
