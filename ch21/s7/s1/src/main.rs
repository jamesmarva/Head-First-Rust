// #![feature(spceialization)]
use std::fmt::Display;

trait Example {
    fn call(&self);
}

impl<T> Example for T {
    default fn call(&self) {
        println!("most generic");
    }
}


impl<T> Example for T
    where T: Display 
{
    default fn call(&self) {
        println!("specialized for str, {}", self)
    }
}

impl Example for str {
    fn call(self: &Self) {
        println!("speciallize for str, {}", self);
    }
}

fn main() {
    let v1 = vec![1i32, 2, 4, 4];
    let v2 = 1i32;
    let v3 = "hello";

    v1.call();
    v2.call();
    v3.call();
}
