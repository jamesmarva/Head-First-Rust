use std::fmt::{Display, Formatter, Result};
use std::cmp::PartialOrd;
use std::cmp::Ordering;

struct T {
    val: i32,
}

impl PartialOrd for T {

    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.val.partial_cmp(&(other.val))
    }
}

impl PartialEq for T {
    fn eq(&self, other: &T) -> bool {
        self.val == other.val
    }
}

impl Display for T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.val)
    }
}

fn max(a: T, b: T) -> T {
    if a < b {
        b
    } else {
        a
    }
}

fn main() {
    let v1 = T {val: 1};
    let v2 = T {val: 2};
    println!("{}", max(v1, v2))
}
