fn main() {
    println!("Hello, world!");

    let add  = |a: i32, b: i32| -> i32 {
        return a + b;
    };
    let x = add(1, 2);

    println!("result is {}", x);

}
