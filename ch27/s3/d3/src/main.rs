fn main() {
    use std::thread;

    let mut v: Vec<i32> = vec![];

    let t = thread::spawn( || {
        v.push(1);
    });
    println!("{:?}", v);
    t.join();

    test1();
    test2();

}

fn test1() {
    let mut v = vec![1];
    let t = std::thread::spawn(move || {
        v.push(1);
    });
}

fn test2() {
    let mut v = vec![1];
    let t = std::thread::spawn(move || {
        v.push(1);
    });
    println!("{:?}", v);
}



