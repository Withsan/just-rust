use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let value = client.get(&key).await;
                    resp.send(value).unwrap();
                }
                Command::Set { key, value, resp } => {
                    let result = client.set(&key, value).await;
                    resp.send(result).unwrap();
                }
            }
        }
    });
    send_command(tx).await;
    manager.await.unwrap();
}
async fn send_command(tx: Sender<Command>) {
    let tx1 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();
        let command = Command::Set {
            key: "123456".to_string(),
            value: "654321".into(),
            resp: res_tx,
        };
        tx1.send(command).await.unwrap();
        let res = res_rx.await.unwrap();
        println!("GOT:{:?}", res);
    });
    let tx2 = tx.clone();
    let t2 = tokio::spawn(async move {
        let (res_tx, res_rx) = oneshot::channel();
        let command = Command::Get {
            key: "123456".to_string(),
            resp: res_tx,
        };
        tx2.send(command).await.unwrap();
        let res = res_rx.await.unwrap().unwrap();
        println!("GOT= {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}
