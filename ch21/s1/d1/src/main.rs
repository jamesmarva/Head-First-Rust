struct S<T=i32> {
    data: T
}

fn main() {
    let v1 = S {data: 0};
    let v2 = S::<bool>{data: false};
    println!("{} {}", v1.data, v2.data);
}

enum MyOption<T> {
    Some(T),
    None,
}


struct Num<T> {
    data: Option<T>
}


