use etcd_client::Error;
mod key_value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    key_value::key_value().await?;
    Ok(())
}
