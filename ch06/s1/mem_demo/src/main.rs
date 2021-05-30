use std::mem::size_of as mem_size_of;
fn main() {
    println!("size of &str: {:?}", mem_size_of::<&str>());
    println!("size of &[i32]: {:?}", mem_size_of::<&[i32]>());
    println!("size of &[u128]: {:?}", mem_size_of::<&[u128]>());
    // println!("size of str: {:?}", mem_size_of::<str>());
    println!("size of String: {:?}", mem_size_of::<String>());
    println!("size of Vec<u128>: {:?}", mem_size_of::<Vec<u128>>());

    println!("---------------------------------------");
    println!("size of &[i32; 10]: {:?}", mem_size_of::<&[i32; 10]>());
    println!("size of &[i32; 1]: {:?}", mem_size_of::<&[i32; 1]>());
    println!("size of &[u128; 1]: {:?}", mem_size_of::<&[u128; 1]>());

    println!("---------- reference of primitive type -------------");
    println!("size of &i32: {:?}", mem_size_of::<&i32>());
    println!("size of &u128: {:?}", mem_size_of::<&u128>());
    println!("------------------------------------------");

    struct Demo {
        fir: i32,
        sec: i32,
    };
    let d = Demo {fir: 1, sec: 2};
    println!("----------- reference of struct type -------------");
    println!("size of &Demo: {:?}", mem_size_of::<&Demo>());
    println!("------------------------------------------");

}
