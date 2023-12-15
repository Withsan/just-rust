use tracing::Level;

fn main() {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    tracing::info!("hello tracing");
    let s = String::from("hello");
    let boxed_s = Box::new(s);
    let leaked_s = Box::leak(boxed_s);
    println!("{}", leaked_s);
}
