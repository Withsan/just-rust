use std::env;
use std::error::Error;

use futures::FutureExt;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let forward_addr = env::args()
        .nth(2)
        .unwrap_or_else(|| "10.4.64.196:8443".to_string());
    println!("listen on {}", listen_addr);
    println!("forward to {}", forward_addr);
    let listener = TcpListener::bind(listen_addr).await?;
    loop {
        let (inbound, client_addr) = listener.accept().await?;
        println!(
            "accept:{:?}, and forward to {:?}",
            client_addr, forward_addr
        );
        let outbound = TcpStream::connect(forward_addr.clone()).await?;
        let transfer = transfer(inbound, outbound).map(|result| {
            if let Err(err) = result {
                eprintln!("err:{:?}", err);
            }
        });
        tokio::spawn(transfer);
    }
}

async fn transfer(mut inbound: TcpStream, mut outbound: TcpStream) -> Result<(), Box<dyn Error>> {
    let (mut read_inbound, mut write_inbound) = inbound.split();
    let (mut read_outbound, mut write_outbound) = outbound.split();
    let client_to_server = async {
        io::copy(&mut read_inbound, &mut write_outbound).await?;
        write_outbound.shutdown().await
    };
    let server_to_client = async {
        io::copy(&mut read_outbound, &mut write_inbound).await?;
        write_inbound.shutdown().await
    };
    tokio::try_join!(client_to_server, server_to_client)?;
    Ok(())
}
