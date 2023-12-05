use std::env;
use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::OnceCell;

static PORT_CELL: OnceCell<&[u8]> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let port = args.into_iter().nth(1).unwrap_or("8080".to_string());
    PORT_CELL.set(port.as_bytes());
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("accept from {:?}", addr);
        tokio::spawn(async move {
            let mut buf = vec![];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err={:?}", e);
                        return;
                    }
                };
                println!(
                    "from client:[{:?}]",
                    &buf.iter()
                        .take(n)
                        .chain(*PORT_CELL.get().unwrap())
                        .map(|by| char::from(*by))
                        .collect::<String>()
                );
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err={:?}", e);
                }
            }
        });
    }
}
