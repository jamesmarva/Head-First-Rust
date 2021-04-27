fn main() {
    let mut v = vec![];
    boo(&mut v);    
}

fn foo(v: &Vec<i32>) {
    // v.push(5);
}

fn boo(v: &mut Vec<i32>) {
    v.push(4)
}


