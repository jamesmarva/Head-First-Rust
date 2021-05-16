fn main() {
    
}

trait Foo {}

trait B {}

trait C {}

impl<T> Foo for T
    where T: B
{

}


impl<T> Foo for T
    where T: C 
{

}

impl<T> Foo for T 
    where T: B + c 
{
    
}
