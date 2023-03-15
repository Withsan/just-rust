use mini_redis::{Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (stream, _client_addr) = listener.accept().await?;
        process(stream).await?;
    }
}
async fn process(stream: TcpStream) -> Result<()> {
    let mut connection = Connection::new(stream);
    if let Some(frame) = connection.read_frame().await? {
        println!("Got: {:?}", frame);
        let response = Frame::Error("fuck you too".to_string());
        connection.write_frame(&response).await?;
    };
    Ok(())
}
