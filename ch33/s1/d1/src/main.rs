use std::str::FromStr;
use std::string::ParseError;
use std::mem::{size_of, size_of_val};


fn main() {
    let r : Result<String, ParseError> = FromStr::from_str("hello");
    println!("Size of String: {}", size_of::<String>());
    println!("Size of 'r': {}", size_of_val(&r));
   
}
