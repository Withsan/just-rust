fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        thread::{self},
        time::Duration,
    };

    use crossbeam::{
        atomic::AtomicCell,
        channel::{bounded, Receiver, Sender},
        queue::ArrayQueue,
        sync::WaitGroup,
    };

    #[test]
    fn atomic_cell() {
        let val = Arc::new(AtomicCell::new(0));
        let mut threads = vec![];
        for _i in 0..10 {
            threads.push(run_by_new_thread(val.clone()));
        }
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
        assert_eq!(10, val.load());
    }
    fn run_by_new_thread(val: Arc<AtomicCell<u32>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            val.fetch_add(1);
        })
    }
    #[test]
    fn array_queue() {
        let queue: Arc<ArrayQueue<usize>> = Arc::new(ArrayQueue::new(100));
        let mut threads = Vec::new();
        for _i in 0..5 {
            threads.push(producer(queue.clone()));
        }
        for _i in 0..5 {
            threads.push(consumer(queue.clone()));
        }
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
        assert_eq!(queue.len(), 0);
    }
    fn consumer(queue: Arc<ArrayQueue<usize>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for _i in 0..5 {
                queue.pop();
            }
        })
    }
    fn producer(queue: Arc<ArrayQueue<usize>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for i in 0..5 {
                queue.push(i).unwrap();
            }
        })
    }
    #[test]
    fn channel_mpmc() {
        let (sender, reciver) = bounded::<usize>(100);
        let mut threads = Vec::new();
        for _ in 0..5 {
            threads.push(producer_channel(sender.clone()));
        }
        for _ in 0..5 {
            threads.push(consumer_channel(reciver.clone()));
        }
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
    }
    fn producer_channel(sender: Sender<usize>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for i in 0..20 {
                sender.send(i).unwrap();
            }
        })
    }
    fn consumer_channel(reciver: Receiver<usize>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..20 {
                let val = reciver.recv().unwrap();
                println!("val is {val}");
            }
        })
    }
    struct Worker {
        wg: WaitGroup,
        job: Box<dyn FnOnce() + Send + 'static>,
    }
    impl Worker {
        fn new(wg: WaitGroup, job: Box<dyn FnOnce() + Send + 'static>) -> Self {
            Self { wg, job }
        }
        fn work(self) {
            (self.job)();
            drop(self.wg);
        }
    }
    #[test]
    fn wait_group() {
        let wg = WaitGroup::new();
        for _ in 0..5 {
            let wg_cloned = wg.clone();
            thread::spawn(move || {
                let worker = Worker::new(wg_cloned, Box::new(|| do_work()));
                worker.work();
            });
        }
        wg.wait();
        println!("work done");
    }
    fn do_work() {
        println!("doing work...");
        thread::sleep(Duration::from_secs(3));
    }
}
