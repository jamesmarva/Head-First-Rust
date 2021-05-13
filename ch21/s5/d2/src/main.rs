use std::iter::Iterator;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result};

fn use_iter<ITEM, ITER>(mut iter: ITER) 
where ITER: Iterator<Item=ITEM>,
    ITEM: Display
{
    while let Some(i) = iter.next() {
        println!("{}", i)
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    use_iter(v.iter())
}
