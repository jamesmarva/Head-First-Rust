use std::iter::Iterator;
use std::fmt::Debug;

fn use_iter<ITEM, ITER>(mut iter: ITER) 
    where ITER: Iterator<Item = ITEM>, 
        ITEM: Debug
{
    while let Some(i) = iter.next() {
        println!("{:?}", i);
    }
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    use_iter(v.iter())
}
