extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let n_workers = 42;

    let n_jobs = 23;
    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    
    for _ in 0..n_jobs {
        let barrier_clone = barrier.clone();
        let an_atomic_clone = an_atomic.clone();

        pool.execute(move || {
            // do the heavy work
            an_atomic_clone.fetch_add(1, Ordering::Relaxed);

            // then wait for the other threads
            barrier_clone.wait();
        });
    }
    barrier.wait();
    println!("{}", an_atomic.load(Ordering::SeqCst));
    assert_eq!(an_atomic.load(Ordering::SeqCst), 23);
}
