fn main() {

    let v = vec![1, 2, 3, 4, 5];
    use_iter(v.iter())
}

use std::iter::Iterator;
use std::fmt::Display;

fn use_iter<ITER, ITEM>(mut iter: ITER) 
    where ITER: Iterator<Item=ITEM>,
        ITEM: Display
{
    while let Some(i) = iter.next() {
        println!("{} ", i);
    }
}
