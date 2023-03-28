use std::error::Error;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = r"C:\Users\Withs\Desktop\heihei.md";
    let content = String::from("hello rust");
    create_file(path).await?;
    write_to_file(path, content.into_bytes()).await?;
    read_from_file(path).await?;
    Ok(())
}
async fn create_file(path: &str) -> Result<(), Box<dyn Error>> {
    let _file = File::create(path).await?;
    Ok(())
}
async fn write_to_file(path: &str, content: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path).await?;
    println!("the content is :{:?}", content);
    let n = file.write(&content[..]).await?;
    println!("Wrote the first {} bytes", n);
    Ok(())
}
async fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path).await?;
    let mut content = Vec::new();
    let n = file.read_to_end(&mut content).await?;
    println!("the bytes {:?}", n);
    Ok(())
}
