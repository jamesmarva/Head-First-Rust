fn main() {
    let v1 = 1;

    let borrow_p = &v1;

    let v2 = *borrow_p;

    println!("{}, {}", v1, v2);
}
