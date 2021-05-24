
fn main() {
    let mut v = vec![1];
    let t = std::thread::spawn(|| {
        v.push(1);
    });
    println!("{:?}", v);
}
