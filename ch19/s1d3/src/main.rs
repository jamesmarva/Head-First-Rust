fn main() {
    let p: Option<&i32> = raw_to_ref(std::ptr::null::<i32>());
    println!("{:?}", p);
}

fn raw_to_ref<'a>(p: *const i32) -> Option<&'a i32> {
    if p.is_null() {
        None
    } else {
        unsafe {
            Some(&*p)
        }
    }
}
