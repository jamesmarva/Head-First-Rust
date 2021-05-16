use std::iter::Iterator;
use std::fmt::Display;


/// 这里要注意， 
/// 这个mut 是表示iter 这个变量是可以被重复赋值，
/// 或者就是表示这个变量指向的内存是可以被修改
fn use_iter<ITEM, ITER> (mut iter: ITER) 
    where ITER: Iterator<Item=ITEM>, 
    ITEM: Display
{
    while let Some(i) = iter.next() {
        println!("{}", i)
    }
}


fn new_iter<ITER>(mut iter: ITER) 
    where ITER: Iterator,
        ITER::Item: Display
{
    // while iter.next().is_some() {
    //     println!("{}", iter.next().unwrap());
    // }

    let mut i: Option<ITER::Item> = iter.next();
    while i.is_some() {
        println!("{}", i.unwrap());
        i = iter.next();
    }
}   

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // use_iter(v.iter())
    new_iter(v.iter())
}
