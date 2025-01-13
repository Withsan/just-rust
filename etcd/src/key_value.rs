use etcd_client::{Client, Error};

pub async fn key_value() -> Result<(), Error> {
    let client = Client::connect(["127.0.0.1:2379"], None).await?;
    let mut kv_client = client.kv_client();
    let key = "hello";
    kv_client.delete(key, None).await?;
    kv_client.put(key, "etcd", None).await?;
    let get_resp = kv_client.get(key, None).await?;
    if let Some(kv) = get_resp.kvs().first() {
        println!("{}:{}", key, kv.value_str()?);
    }
    Ok(())
}
