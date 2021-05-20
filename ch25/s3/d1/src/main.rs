

fn collector() -> Vec<u64> {
    let mut res = vec![];
    let mut curr: u64 = 1;
    let mut next: u64 = 1;
    loop {
        let new_next = curr.checked_add(next);

        if let Some(new_nt) = new_next {
            curr = next;
            next = new_nt;
            res .push(curr);
        } else {
            break;
        }
    }

    return res
}

fn main() {
    let mut v = collector();

    // 
    let mut iter = v.iter();
    while let Some(i) = iter.next() {
        println!("{}", i);
    }

    let mut iter_mut = v.iter_mut();
    while let Some(i)  = iter_mut.next() {
        println!("{}", i);
    }

    let mut iter_key = v.into_iter();
    while let Some(i) = iter_key.next() {
        println!("{}", i);
    }
}
