use std::iter::FromIterator;

fn main() {
    let a = [1i32, 2, 3, 4, 5];
    print_slice(&a);
    let sli1 = &a[2..];
    print_slice(sli1);

    let sli2  = &a[..2];
    print_slice(sli2);

    // print_100_to_10();

    // print_idx_and_val();
    pri_vec_idx_and_val();
}


fn print_slice(arr: &[i32]) {
    println!("lenght: {}", arr.len());
    for i in arr {
        print!("{}\t", i);
    }
    println!("");
}


fn print_100_to_10() {
    let r = (1u32..=10)
        .rev()
        .map(|i| i*10);
    for i in r {
        print!("{}\t", i);
    }
}


fn print_idx_and_val() {
    // let v = &[10u32, 20, 30, 40, 50];
    let v1 = (1u32..=10).map(|i| i * 10);
    let v1_clone = v1.clone();
    for (idx, item) in v1.enumerate() {
        println!("idx: {}; item: {}", idx, item);
    }

    let second_item = v1_clone
        .filter(|x| *x % 2 == 0)
        .nth(2);
    if let Some(x) = second_item {
        println!("{}", x);
    }
}

fn pri_vec_idx_and_val() {
    // let sli = (1u32..=10);
    // let v = vec![1, 2, 3, 4, 5];
    let v1 = Vec::from_iter(1u32..=10);
    for (i, e) in v1.iter().enumerate() {
        println!("i: {}; e: {}", i, e);
    }

    let sec = v1.iter().filter(|x| *x % 2 == 0).nth(0);
    if let Some(val) = sec {
        println!("sec: {}", val);
    }
}

fn pri_sli_i_and_ele() {
    let sli = (1u32..=10);
    for (i, e) in sli.enumerate() {
        
    }
}