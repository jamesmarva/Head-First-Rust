fn main() {
    println!("Hello, world!");
}


pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}


pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
