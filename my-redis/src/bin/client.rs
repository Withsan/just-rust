use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key } => {
                    let _value = client.get(&key).await;
                }
                Command::Set { key, value } => {
                    let _result = client.set(&key, value).await;
                }
            }
        }
    });
    prosess(tx).await;
    manager.await.unwrap();
}
async fn prosess(tx: Sender<Command>) {
    let tx1 = tx.clone();
    let t1 = tokio::spawn(async move {
        let command = Command::Set {
            key: "123456".to_string(),
            value: "654321".into(),
        };
        tx1.send(command).await.unwrap();
    });
    let tx2 = tx.clone();
    let t2 = tokio::spawn(async move {
        let command = Command::Get {
            key: "123456".to_string(),
        };
        tx2.send(command).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: Bytes },
}
