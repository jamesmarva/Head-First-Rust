fn main() {
    let mut v = vec![1];
    std::thread::spawn(move || {
        v.push(2);
    });
    println!("{:?}", v);
}
