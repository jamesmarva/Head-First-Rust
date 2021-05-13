use std::iter::Iterator;
use std::fmt::Display;

fn all<ITER>(mut iter: ITER) 
where ITER: Iterator,
    ITER::Item: Display
{
    while let Some(i) = iter.next() {
        println!("{}", i);
    }
}

fn all_1<ITER, ITEM>(mut iter: ITER) 
where ITER: Iterator<Item=ITEM>,
    ITEM: Display
{
    while let Some(i) = iter.next() {
        println!("{}", i);
    }
}


fn main() {
    let v = vec![1, 2, 3, 4, 5];
    all(v.iter());

}
