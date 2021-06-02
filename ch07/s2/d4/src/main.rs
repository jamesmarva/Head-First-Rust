fn main() {
    let x = (1, 2, 3);
    let (a, ..) = x;
    println!("{}", a);

    let (.., c) = x;
    println!("{}", c);

    let (a, .., c) = x;
    println!("{}, {}", a, c);
}
