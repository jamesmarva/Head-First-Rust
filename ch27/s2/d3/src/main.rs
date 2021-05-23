use std::thread;
fn main() {

    let mut health = 12;

    thread::spawn( || {
        health *= 2;
    });
    println!("{}", headlth);

}
