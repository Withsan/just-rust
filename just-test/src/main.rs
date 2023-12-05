<<<<<<< HEAD
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let r = &v;
    println!("{}", r[0]);
    let aside = v;
=======
use tracing::Level;

fn main() {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    tracing::info!("fuck");
>>>>>>> 1b98085 (test etcd)
}
