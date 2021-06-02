fn main() {
    struct P(f32, f32, f32);

    fn cal(arg: P) -> f32 {
        let P(x, _, y) = arg;
        x * x + y * y
    }

    let t = P(1.0, 2.0, 3.0);
    println!("{}", cal(t));

}
