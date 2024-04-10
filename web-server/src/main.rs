use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crossbeam_channel::{bounded, Receiver, Sender};

fn main() {
    let tcp_listenr = TcpListener::bind("127.0.0.1:1993").unwrap();
    let pool = ThreadPool::new(10);
    for stream in tcp_listenr.incoming() {
        let stream = stream.unwrap();
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
type Job = Box<dyn FnOnce() + Send + 'static>;
struct ThreadPool {
    _workers: Vec<Worker>,
    jobs: Sender<Job>,
}
impl ThreadPool {
    fn new(num: usize) -> ThreadPool {
        let mut workers = vec![];
        let (sender, receiver) = bounded(num);
        (0..num).for_each(|id| {
            workers.push(Worker::new(id, receiver.clone()));
        });
        ThreadPool {
            _workers: workers,
            jobs: sender,
        }
    }
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.jobs.send(Box::new(f)).unwrap();
    }
}
struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Receiver<Job>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.recv().unwrap();
            println!("Worker {id} got a job");
            job();
        });
        Worker {
            _id: id,
            _thread: thread,
        }
    }
}
