use etcd_client::Error;
mod key_value;
mod lease;

#[tokio::main]
async fn main() -> Result<(), Error> {
    key_value::key_value().await?;
    lease::lease().await?;
    Ok(())
}
