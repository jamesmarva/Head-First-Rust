extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {

    let n_workers = 4;
    let n_jobs = 8;

    let pool = ThreadPool::new(n_workers);

    let (sender, receiver) = channel();

    for _ in 0..n_jobs {
        let new_sender = sender.clone();
        pool.execute(move ||{
            new_sender.send(1).expect("channel wil be there waitging for the pool.");
        });
    }

    println!("{}", receiver.iter().take(n_jobs).fold(0, |a, b| a + b));
}
