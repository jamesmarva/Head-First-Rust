fn main() {
    //  as 进行类型转换
    let mut f1 = add1 as fn((i32, i32)) ->i32;
    let mut f2 : fn((i32, i32)) -> i32 = add1;
    f1 = f2;
}
fn add1(p: (i32, i32)) -> i32 {
    p.0 + p.1
}
fn add2((x, y) : (i32, i32)) -> i32 {
    x + y
}

