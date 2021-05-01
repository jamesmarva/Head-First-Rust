fn main() {

    use std::rc::Rc;
    use std::ops::Deref;

    let s = Rc::new(String::from("hello"));

    // 以下的解法在编译器看来都是一样的。
    println!("length: {}", s.len());
    println!("length: {}", s.deref().len());
    println!("length: {}", s.deref().deref().len());

    println!("length: {}", (*s).len());
    println!("length: {}", (&*s).len());
    println!("length: {}", (&**s).len());
}
