/// 借用指针类型的都有一个生命周期泛型参数。
struct T {
    member: i32,
}

fn test<'a>(arg: &'a T) -> &'a i32 
{
    &arg.member
}

fn main() {
    let t = T{member: 0};
    let x = test(&t);
    println!("{:?}", x);
}

