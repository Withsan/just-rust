use std::{fs, io::Write};

use tracing::Level;

// #[tokio::main]
fn main() {
    let subscribe = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscribe).expect("error");
    let mut data = [2, 2];
    println!("{:?}", &data[..]);
    println!("{:?}", "fuck".as_bytes());
    let mut file = fs::File::create("data.txt").unwrap();
    file.write_all(&data).unwrap();
    // write!(file, "{:?}", &data).unwrap();
}
