use etcd_client::{Client, Error, EventType};

pub async fn watch() -> Result<(), Error> {
    let etcd_client = Client::connect(["127.0.0.1:2379"], None).await?;
    let mut watch_client = etcd_client.watch_client();
    let key = "hello";
    let (mut watcher, mut watch_stream) = watch_client.watch(key, None).await?;
    tracing::info!("create watcher {}", watcher.watch_id());
    while let Some(resp) = watch_stream.message().await? {
        tracing::info!("[{}]receive watch response", resp.watch_id());
        if resp.created() {
            tracing::info!("watcher created :{}", resp.watch_id());
        }
        if resp.canceled() {
            tracing::info!("wtach canceled:{}", resp.watch_id());
        }
        for event in resp.events() {
            tracing::info!("event type:{:?}", event.event_type());
            if let Some(kv) = event.kv() {
                tracing::info!("kv:{{{}:{}}}", kv.key_str()?, kv.value_str()?);
            }
        }
    }
    Ok(())
}
