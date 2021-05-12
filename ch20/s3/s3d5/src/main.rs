fn main() {
    
}
struct Unique<T> {
    val: T
}

unsafe impl<T: Send+?Sized> Send for Unique<T> {

}


unsafe impl<T: Send+?Sized> Sync for Unique<T> {
    
}


