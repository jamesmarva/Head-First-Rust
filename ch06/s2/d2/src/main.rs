fn main() {
    let mut s = String::from("Hello James");
    s.push(' ');
    s.push_str("hhhhhhhhhhhhhhhhhhhhhhh");
    println!("{}", s);
    demo1();
}

fn demo1() {
    let mut s = String::from("hello james");
    /// &mut String auto cast to &mut str
    uppercase(&mut s);
    println!("{}", s);
}

fn uppercase(s: &mut str)  {
    s.make_ascii_uppercase();
}