use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mini_redis::{Command, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (stream, _client_addr) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move { process(stream, db).await });
    }
}
async fn process(stream: TcpStream, db: Db) -> Result<()> {
    let mut connection = Connection::new(stream);
    while let Some(frame) = connection.read_frame().await? {
        println!("Got: {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await?;
    }
    Ok(())
}
