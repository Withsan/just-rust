use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("fuck", "you".into()).await?;
    println!("nice here");
    if let Some(result) = client.get("fuck").await? {
        println!("got value from the mini-redis-server:{:?}", result);
    };
    Ok(())
}
