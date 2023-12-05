use etcd_client::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let etcd_client = Client::connect(["127.0.0.1:2379"], None).await?;
    let mut kv_client = etcd_client.kv_client();
    let response = kv_client.get("fuck", None).await?;
    for ele in response.kvs() {
        println!("Get {{{}:{}}}", ele.key_str()?, ele.value_str()?);
    }
    Ok(())
}
