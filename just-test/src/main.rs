use std::error::Error;
use std::sync::Arc;

use tokio::task::yield_now;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async {
        let rc = Arc::new("hello");
        println!("rc:{rc}");
        yield_now().await;
    });
    Ok(())
}
