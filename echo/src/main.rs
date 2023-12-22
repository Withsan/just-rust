use std::env;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let boxed_port = args.nth(1).unwrap().into_boxed_str();
    let port: &'static str = Box::leak(boxed_port);
    let mut server_addr = String::from("127.0.0.1:");
    server_addr.push_str(&port);
    println!("Echo server listen on {}", &server_addr);
    let listener = TcpListener::bind(server_addr.clone()).await?;
    let server_addr: &'static str = Box::leak(server_addr.into_boxed_str());
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("accept from {:?}", addr);
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("read zero byte!");
                        return;
                    }
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
                        .map(|by| char::from(*by))
                        .collect::<String>()
                );

                let mut from_str = String::from(" from: ");
                from_str.push_str(&server_addr.to_string().as_str());
                if let Err(e) = socket
                    .write_all(&[&buf[0..n], &from_str.as_str().as_bytes()[..]].concat()[..])
                    .await
                {
                    eprintln!("failed to write to socket; err={:?}", e);
                }
            }
        });
    }
}
