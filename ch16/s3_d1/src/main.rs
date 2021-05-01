fn main() {
    use std::rc::Rc;

    let s = Rc::new(String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"));
    println!("{:?}", s.bytes());
}
