extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {

    let n_workers = 4;
    let n_jobs = 8;

    let pool = ThreadPool::new(n_workers);
    let (s, r) = channel();
    for _ in 0..n_jobs {
        let s_clone = s.clone();
        pool.execute(move || {
            s_clone.send(1).expect("channel will be there waiting for the pool");
        });
        assert_eq!(r.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
    }
}
