fn main() {

    let p = (1, 3);
    let func = add2;
    // 函数可以当成头等公民（first class value）
    // 被复制到一个值中，这个值可以像函数一样被调用
    println!("{}", func(p));
}

fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y) : (i32, i32)) -> i32{
    x + y
}
