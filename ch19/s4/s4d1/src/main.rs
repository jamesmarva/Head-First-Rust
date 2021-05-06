struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let mut x = Foo {a: 0, b: 1, c: 2};
    let pa = &mut x.a;
    let pb = &mut x.b;
    let pc = &x.c;

    *pb += 1;
    let pc2 = &x.c;
    *pa += 10;
    println!("{}, {}, {}, {}", pa, pb, pc, pc2);
}
