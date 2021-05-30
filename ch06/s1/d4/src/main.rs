use std::mem::size_of as mem_size_of;

fn main() {
    println!("size of [i32; 1]: {}", mem_size_of::<[i32; 1]>());
    println!("size of [i32; 4]: {}", mem_size_of::<[i32; 4]>());
    println!("size of [i32; 5]: {}", mem_size_of::<[i32; 5]>());
    println!("size of [i32; 6]: {}", mem_size_of::<[i32; 6]>());
    println!("-------------------------------------------");
    println!("size of &[i32; 3]: {:?}", mem_size_of::<&[i32; 3]>());
    println!("size of &[i32; 10]: {:?}", mem_size_of::<&[i32; 10]>());
    println!("--------------------------------------------");
    println!("size of &[i32]: {:?}", mem_size_of::<&[i32]>());
    println!("size of &[i64]: {:?}", mem_size_of::<&[i64]>());
    println!("size of vec<i32>: {:?}", mem_size_of::<Vec<i32>>());
    println!("--------------------------------------------");
    println!("size of &i32: {:?}", mem_size_of::<&i32>());
    println!("size of &u64: {:?}", mem_size_of::<&u64>());
    println!("size of &u128: {:?}", mem_size_of::<&u128>());
    println!("size of Box<i32>: {:?}", mem_size_of::<Box<i32>>());
}

fn test() {
    let v = [1u32; 5];
    let mut v = [1i32; 3];
    {
        let s = &mut v as &mut [i32];
        // 如果没有这一句的话，那么就会编译通过，也就说，根据上下文的，
        // 如果没有没有类型转换的话，那么借用数组还是有长度的信息
        s[10] = 111;
        mut_arr(s);
        // mut_arr(s);
    }
    println!("{:?}", v);
}
fn mut_arr(a: &mut [i32]) {
    a[10] = 5;
}
