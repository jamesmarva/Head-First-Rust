fn main() {
    let mut x = 1_i32;
    println!("x address: {:p}", &x);
    let mut y: u32 = 1;
    println!("y address: {:p}", &y);

    let mut z = 1_i32;
    println!("z address: {:p}", &z);

    let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i128;

    unsafe {
        // 此时，我们对该指针所指向的地址进行修改会发生“类型安全”问题以及“内存安全”问题。
        *raw_mut = -1;
        println!("{}", *raw_mut);
    }

    // 补码的16进制的写法？
    println!("{:X} {:X}", x, y);
    println!("y address: {:p}", &y);

    println!("{:p}", &x);
}
