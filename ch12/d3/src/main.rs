fn main() {
    let mut v = vec![];

    foo(&mut v);

    println!("{:?}", v);
    println!("{:?}", v);
    }

fn foo (v: &mut Vec<i32>) {
    v.push(4)
}


