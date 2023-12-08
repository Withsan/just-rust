use std::time::Duration;

use etcd_client::{Client, Error, PutOptions};

pub async fn lease() -> Result<(), Error> {
    let client = Client::connect(["127.0.0.1:2379"], None).await?;
    let resp = client.lease_client().grant(30, None).await?;
    let id = resp.id();
    let put_opt = PutOptions::new();
    let put_opt = put_opt.with_lease(id);
    client.kv_client().put("my", "etcd", Some(put_opt)).await?;
    for _ in 0..4 {
        tokio::time::sleep(Duration::from_secs(10)).await;
        let resp = client.kv_client().get("my", None).await?;
        if let Some(key_value) = resp.kvs().first() {
            println!("{}", key_value.value_str()?);
        } else {
            println!("no");
        }
    }
    Ok(())
}
