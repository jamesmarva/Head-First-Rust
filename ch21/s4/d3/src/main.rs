use std::fmt::{Display, Formatter, Result};
use std::cmp::{PartialOrd, Ordering};

struct T {
    value: i32,
}

fn max<T>(a: T, b: T) -> T 
    where T: PartialOrd
{
    if a > b {
        a
    } else {
        b
    }
}

impl PartialOrd for T {
    fn partial_cmp(self: &Self, o: &T) -> Option<Ordering> {
        self.value.partial_cmp(&(o.value))
    }
}

impl PartialEq for T {
    fn eq(self: &Self, o: &T) -> bool {
        self.value == o.value
    }
}

impl Display for T {

    fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result {
        let s: String = String::from("T");
        // s.append()
        write!(f, "{}", self.value)
    }
    
}

fn main() {

    let a = T {value: 1};
    let b = T {value: 2};
    println!("{}", max(a, b))
    
}

