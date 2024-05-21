use std::{error::Error, fs::File, io::Read};

fn main() {
    println!("Hello, world!");
}
fn read_to_string() -> Result<usize, Box<dyn Error>> {
    let mut buf = String::new();
    File::open("fuck.txt").err().unwrap().into_inner()
}
