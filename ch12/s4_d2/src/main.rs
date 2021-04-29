fn main() {
    let t = T {member: 0};
    let x = test(&t);
    println!("{}", x);

    let x1 = t1(&t);
    println!("x1: {}", x1);
}

struct T {
    member: i32,
}

fn test<'a>(arg: &'a T) -> &'a i32 {
    &arg.member
}

fn t1<'a, 'b>(arg: &'a T) -> &'a i32 
    where 'a: 'b
{
    &arg.member
}