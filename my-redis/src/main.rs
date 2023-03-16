use std::collections::HashMap;

use mini_redis::{Command, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (stream, _client_addr) = listener.accept().await?;
        tokio::spawn(async move { process(stream).await });
    }
}
async fn process(stream: TcpStream) -> Result<()> {
    let mut db: HashMap<String, Vec<u8>> = HashMap::new();
    let mut connection = Connection::new(stream);
    while let Some(frame) = connection.read_frame().await? {
        println!("Got: {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await?;
    }
    Ok(())
}
