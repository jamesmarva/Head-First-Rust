use std::mem::size_of as mem_size_of;


fn main() {
    println!("{}", mem_size_of::<&[i32]>());
    println!("{}", mem_size_of::<&[i128]>());
    println!("{}", mem_size_of::<&i32>());
    println!("{}", mem_size_of::<&i64>());
    println!("{}", mem_size_of::<&i128>());


    struct D {
        a: i64,
        b: i64,
    }
    // let d = D {a: 11, b: 11};
    println!("size of type D: {}", mem_size_of::<D>());
    println!("size of reference of D: {}", mem_size_of::<&D>());
    println!("-----------------------------");
    println!("{}", mem_size_of::<i32>());
    println!("{}", mem_size_of::<i64>());
    println!("{}", mem_size_of::<i128>());
}
