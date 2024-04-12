use tracing::Level;

#[tokio::main]
async fn main() {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    let second = 2;
    {
        let first = 3;
        let result = two(&first, &second).await;
        println!("{result}")
    }
}
async fn two<'a: 'b, 'b>(first: &'a i32, sencond: &'b i32) -> i32 {
    first * sencond
}
