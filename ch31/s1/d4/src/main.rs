extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};
/// https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
/// 
/// 
/// 
/// 
fn main() {

    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (sender, receiver) = channel();
    for _ in 0..n_jobs {
        let sender_clone = sender.clone();
        pool.execute(move || {
            sender_clone.send(1).expect("channel will be there waiting for the pool");
        });
    }

    println!("{}", receiver.iter().take(7).fold(0, |a, b| a + b));

    barrier_thrreadpool();
}

fn barrier_thrreadpool () {
    let n_workers = 42;
    let n_jobs = 23;

    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    let barrier = Arc::new(Barrier::new(n_jobs + 1));

    for _ in 0..n_jobs {
        let barrier_coe = barrier.clone();
        let an_atomic_coe = an_atomic.clone();

        pool.execute(move || {
            an_atomic_coe.fetch_add(10, Ordering::Relaxed);
            println!("waiting ....");
            barrier_coe.wait();
            println!("after ....");
        });
    }
    barrier.wait();
    println!("{}", an_atomic.load(Ordering::Relaxed));
}
