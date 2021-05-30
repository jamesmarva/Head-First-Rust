fn main() {
    let p1 = "hello";
    let p2 = p1.to_string();

    let o = p1;
    println!("first o : {:?}", o);

    let o = p2;
    println!("second o: {:?}", o);
}
