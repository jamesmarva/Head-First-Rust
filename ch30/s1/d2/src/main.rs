use std::sync::mpsc::channel;

fn main() {

    let (sender, receiver) = channel();
    for i in 0..10 {
        let n_sender = sender.clone();
        std::thread::spawn(move || {
            n_sender.send(i).unwrap();
        });
    }
    drop(sender);
    while let Ok(rec) = receiver.recv() {
        println!("received  {}", rec);
    }
}
