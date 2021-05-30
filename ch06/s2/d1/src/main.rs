fn main() {
    demo2();
}

fn deom1() {
    let greeting = "Hello James";
    let subgre = &greeting[2..];
    println!("{}", subgre);
}

fn demo2() {
    let ch = "我是天才，哈哈哈哈哈哈哈哈";
    let mut sub_ch = ch.char_indices();
    // println!("{:?}", sub_ch.next());
    // println!("{:?}", sub_ch.next());
    while let Some((i, c)) = sub_ch.next() {
        println!("i: {}; c: {}", i, c);
    }
    // println!("{}", sub_ch);
}
