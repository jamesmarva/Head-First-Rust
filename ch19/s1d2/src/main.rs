
// 。这段代码里面直接用unsafe功能把一个裸指针转换为了一个共享引用，
// 忽略了Rust里面共享引用必须遵循的规则
fn raw_to_ref<'a>(p: *const i32) -> &'a i32 {
    unsafe {
        &*p
    }
}
fn main() {
    let p: &i32 = raw_to_ref(std::ptr::null::<i32>());
    println!("{}", p);
}
