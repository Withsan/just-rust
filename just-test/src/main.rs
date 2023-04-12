use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut iter = tokio_stream::iter(&[1, 23, 4, 5, 6]);
    while let Some(item) = iter.next().await {
        println!("it is {:?}", item);
    }
}
