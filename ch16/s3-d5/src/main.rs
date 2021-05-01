fn main() {

    let s = Box::new(String::from("hhhhh"));

    let point = &*s;

    println!("{} {}", point, s);
}

fn joint() {
    let s = Box::new(String::from("hhh"));

    // point : &str
    let point = &*s;

    println!("{}", point);
}

fn separate() {
    let s = Box::new(String::new());

    // tmp : String 
    let tmp = *s;

    let point = &tmp;

    // move occurs because `*s` has type `std::string::String`,
    // which does not implement the `Copy` 
    println!("{}, {}", point, s);
}
