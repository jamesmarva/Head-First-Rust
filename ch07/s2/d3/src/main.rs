fn main() {

    let p = P(2.0, 3.0, 4.0);
    print!("{}", cal(p));
}


struct P(f32, f32, f32);

fn cal(P(x, _, y): P) -> f32 {
    x * x + y * y
}