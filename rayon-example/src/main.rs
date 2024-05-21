fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread, time::Duration};

    use rand::{thread_rng, Rng};
    use rayon::{prelude::*, ThreadPoolBuilder};
    #[test]
    fn for_each() {
        (0..1000)
            .into_par_iter()
            .for_each(|item| println!("{:?}", item));
    }
    #[test]
    fn for_each_with() {
        let (sender, reciver) = channel();
        (0..1000)
            .into_par_iter()
            .for_each_with(sender, |sender, item| sender.send(item).unwrap());
        let mut result = reciver.into_iter().collect::<Vec<_>>();
        result.sort();
        for ele in result {
            println!("{ele}")
        }
    }
    #[test]
    fn for_each_init() {
        let mut vec = vec![0u8; 1_000_000];
        vec.par_chunks_mut(1000)
            .for_each_init(|| thread_rng(), |rng, chunk| rng.fill(chunk));
        for i in 0..=255 {
            assert!(vec.contains(&i));
        }
    }
    #[test]
    fn thread_pool() {
        let thread_pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();
        for _ in 0..10000 {
            thread_pool.install(|| {
                println!("{:?}", thread::current().id());
            });
        }
    }
}
