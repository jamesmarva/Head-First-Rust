fn main() {
    let mut f1 = add1;
    f1 = add3;
}

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y) : (i32, i32)) -> i32 {
    x + y
}

fn add3(t: (i32, i32)) -> i32 {
    t.0 + t.1
}
