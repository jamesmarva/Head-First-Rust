use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

const COUNT: u32 = 100000;

fn main() {
    let global = Arc::new(Mutex::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move|| {
        for _ in 0..COUNT {
            match clone1.lock() {
                Ok(mut value) => *value += 1,
                Err(poisoned) => {
                    let mut value = poisoned.into_inner();
                    *value += 1;
                }
            }
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut val = clone2.lock().unwrap();
            *val -= 1;
            if *val < 100000 {
                println!("make a panic");
                panic!("");
            }
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    println!("final value {:?}", global);
}