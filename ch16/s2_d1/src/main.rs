fn main() {
    let s = "hello";
    println!("length: {}", s.len());
    println!("length: {}", (&s).len());

    println!("length: {}", (&&&&&&s).len());

    println!("UFCS length: {}", str::len(&s));
}
