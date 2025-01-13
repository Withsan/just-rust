use std::time::Duration;

use etcd_client::{Client, Error};

pub async fn lock() -> Result<(), Error> {
    let etcd_client = Client::connect(["127.0.0.1:2379"], None).await?;
    let mut lock_client = etcd_client.lock_client();
    let key = "lock";
    if let Ok(resp) = lock_client.lock(key, None).await {
        let key = resp.key();
        let key_str = std::str::from_utf8(resp.key());
        tracing::info!("locked key :{:?}", key_str);
        tokio::time::sleep(Duration::from_secs(10)).await;
        lock_client.unlock(key).await?;
    }
    Ok(())
}
