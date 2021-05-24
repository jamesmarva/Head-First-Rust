use std::sync::mpsc::channel;

fn main() {

    let (s, r) = channel();
    let new_s = s.clone();
    for i in 0..10 {
        new_s.send("new sender").unwrap();
    }

    for i in 0..10 {
        s.send("old sender").unwrap();
    }

    while let Ok(rec) = r.recv() {
        println!("{}", rec);
    }
}
