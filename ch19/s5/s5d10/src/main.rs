use std::fmt::Debug;

#[derive(Clone, Debug)]
struct S;


#[derive(Debug)]
struct  R<T: Debug> {
    x: *const T
}

// 上面这种简单的写法是有问题的，因为我们可以很容易制造出悬空指针
fn main() {
    let mut r = R {x: std::ptr::null() };
    {
        let local = S{};
        r.x = &local;
    }
    
    // r.x now is dangling pointer
}
