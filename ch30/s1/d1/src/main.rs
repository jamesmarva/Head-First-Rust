fn main() {
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    std::thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    while let Ok(r) = rx.recv() {
        println!("received {}", r);
    }
}
