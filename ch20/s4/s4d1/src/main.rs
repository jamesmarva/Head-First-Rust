fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("{:?} ", get(&arr, 2));
}

fn get(arr: &[u8], idx: usize) -> Option<u8> {
    if idx < arr.len() {
        unsafe {
            // 取地址，然后用偏移量来取值
            let p = arr.as_ptr().offset(idx as isize);
            // 
            Some(*p)
        }
    } else {
        None
    }
}
