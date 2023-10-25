use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let tcp_listenr = TcpListener::bind("127.0.0.1:1993").unwrap();
    let pool = ThreadPool::new(10);
    for stream in tcp_listenr.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let resopnse = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(resopnse.as_bytes()).unwrap();
}
struct ThreadPool;
impl ThreadPool {
    fn new(num: usize) -> ThreadPool {
        ThreadPool
    }
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
