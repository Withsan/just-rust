use std::time::Duration;

use etcd_client::{Client, Error, PutOptions};

pub async fn lease() -> Result<(), Error> {
    let client = Client::connect(["127.0.0.1:2379"], None).await?;
    let id = client.lease_client().grant(30, None).await?.id();
    let put_opt = PutOptions::new().with_lease(id);
    let key = "my";
    let value = "etcd";
    client.kv_client().put(key, value, Some(put_opt)).await?;
    for _ in 0..4 {
        tokio::time::sleep(Duration::from_secs(10)).await;
        let resp = client.kv_client().get(key, None).await?;
        if let Some(key_value) = resp.kvs().first() {
            println!("{}:{}", key_value.key_str()?, key_value.value_str()?);
        } else {
            println!("key:{} not found", key);
        }
    }
    Ok(())
}
