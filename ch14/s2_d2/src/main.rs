/// & 是个借用指针
fn main() {
    let mut dt = 100i32;

    let mut p = &dt;

    println!("{}", p);

    dt = 111;
    // 注意，这里是不能再次读取的，因为，如果这里再次读取，那么上一行就是错的了。
    // 因为dt 在此被assign之后，原来的存在内存的已经消失了。
    // 
    // println!("{}", p);
    p = &dt;
    println!("{}", p);
}
