use std::thread;
fn main() {

    let mut h = 12;

    let t = thread::spawn(move || {
        h *= 2;
        println!("in thread {}", h);
    });
    // t.join();
    println!("{}", h);
    t.join();
}
