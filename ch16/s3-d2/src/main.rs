use std::rc::Rc;

fn main() {

    let s = Rc::new(String::from("hello"));

    // Rc 本身没有 bytes() 这方法的，编译器会去尝试先自动 deref，也就是自动尝试 s.deref().bytes()
    // 但是 String 也没有 bytes 方法，所以会继续尝试 deref()，这样就会尝试 s.deref().deref().bytes()
    // String deref之后就是 str，所以String也是个智能指针？
    // 所以这次实际上是调用了 str.bytes() 。
    println!("{:?}", s.bytes());

}
