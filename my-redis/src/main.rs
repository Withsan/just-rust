use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

type ShardDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let shard_db = new_shard_db(10);
    loop {
        let (stream, _client_addr) = listener.accept().await?;
        let shard_db = shard_db.clone();
        tokio::spawn(async move { process(stream, shard_db).await });
    }
}
async fn process(stream: TcpStream, shard_db: ShardDb) -> Result<()> {
    let mut connection = Connection::new(stream);
    while let Some(frame) = connection.read_frame().await? {
        println!("Got: {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Command::Get(cmd) => {
                let key = cmd.key();
                let db = shard_db
                    .get(get_key_hash(key, shard_db.len()))
                    .unwrap()
                    .lock()
                    .unwrap();
                if let Some(value) = db.get(key) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            Command::Set(cmd) => {
                let key = cmd.key();
                let mut db = shard_db
                    .get(get_key_hash(key, shard_db.len()))
                    .unwrap()
                    .lock()
                    .unwrap();
                db.insert(key.to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await?;
    }
    Ok(())
}
fn new_shard_db(num_shards: usize) -> ShardDb {
    let mut shard_db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        shard_db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(shard_db)
}
fn get_key_hash(key: &str, len: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_code: usize = hasher.finish() as usize;
    hash_code % len
}
