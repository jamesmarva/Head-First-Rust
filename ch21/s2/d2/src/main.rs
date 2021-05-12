fn main() {
    let o1 = Some(1i32);
    let o2 = Some(1u8);
    let s1 = Some(..);
    println!("{}", compare_option(o1, o2));
}


fn compare_option<T1, T2>(fir: Option<T1>, sec: Option<T2>) -> bool {
    match(fir, sec) {
        (Some(u8), Some(..)) => true,
        (None, None) => true,
        _ => false
    }
}