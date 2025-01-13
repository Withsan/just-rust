use tracing::Level;

#[tokio::main]
async fn main() {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    let my_str = String::from("fuck");
    let fn_once_closure = || {
        let your_str = my_str;
        println!("{}", your_str);
    };
    fn_once_closure();
}
