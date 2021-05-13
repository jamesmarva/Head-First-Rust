use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

fn max<T>(a: T, b: T) -> T
    where T: PartialOrd
{
    if a < b {
        b
    } else {
        a
    }
}

struct T {
    value: i32
}

impl PartialOrd for T {

    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for T {

    fn eq(self: &Self, other: &T) -> bool {
        self.value == other.value
    }
}

impl Display for T {

    fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let t1 = T {value: 1};
    let t2 = T {value: 2};
    let m = max(t1, t2);
    // println!("t1: {}", t1);
    println!("max: {}", m);
}