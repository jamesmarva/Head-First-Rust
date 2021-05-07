fn main() {
    println!("Hello, world!");
}

use std::marker::PhantomData;


//  你还可以用它来表明当前这个类型不可Send、Sync
struct MyStruct {
    data: String,
    _marker: PhantomData<*mut()>,
}

// 
struct MyBorrowStruct <'a, T: ?Sized>{
    data: String,
    _marker: PhantomData<&'a T>,
}


// struct Unique<T: ?Sized> {
    // pointer: NonZero<*const T>,
//     _marker: PhantomData<*mut ()>,
// }


struct BorrowStruct<'a, T: ?Sized> {
    data: String,
    _marker: PhantomData<&'a T>
}

struct OwnerStruct<T: ?Sized> {
    data: String,
    _marker: PhantomData<T>
}





