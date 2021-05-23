use std::thread;

fn main() {
    let mut h = 12;
    thread::spawn( || {
        h *= 2;
    });

    println!("{}", h);
}
