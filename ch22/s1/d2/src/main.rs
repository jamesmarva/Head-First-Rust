fn main() {
    println!("Hello, world!");

    let add = |a: i32, b: i32| -> i32 {
        return a + b;
    };

    let add1 = |c, d| -> i32 {
        return c + d; 
    };

    let rst = add1(1, 2);

    let add2 = |a, b| -> i32 {a + b};

    let rst = add2(1, 33);

    let add3 = |g, f| g + f;

    let rst: f64 = add3(6.1, 1.1);
    println!("add3 rst{}", rst);
}
