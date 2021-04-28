/// mut 如果绑定的是变量，就表示变量能被重新帮
/// mut 如果修饰的是 引用，也就是“借用指针”，那么代表着，这个指针指向的对象可以被修改。
fn main() {
    let mut var = 0_i32;
    {
        let p1 = &mut var;
        *p1 = 1;
        println!("var {}", var);
    }

    {
        let temp  = 2_i32;
        let mut p2 = &var;
        p2 = &temp;
        println!("p2 {}", p2);
    }
    println!("var {}", var);

    {
        let mut temp = 3_i32;
        let mut p3 = &mut var;
        *p3 = 4_i32;
        println!("p3 {}", p3);

        p3 = &mut temp;
        println!("p3 {}", p3);

    }
    println!("var {}", var);
}


