use etcd_client::{Client, Error};

pub async fn key_value() -> Result<(), Error> {
    let client = Client::connect(["127.0.0.1:2379"], None).await?;
    let mut kv_client = client.kv_client();

    let resp = kv_client.get("fuck", None).await?;
    if let Some(kv) = resp.kvs().first() {
        println!("{}", kv.value_str()?);
    }
    Ok(())
}
