use core::prelude;


fn main() {

    let mut xs = [1u32, 5];
    let ys = [2u32, 5];

    xs = ys;
    println!("{:?}", xs);
    xs[0] = 333;
    println!("{:?}", xs);

    // for i in xs {
    //     println!("{}", i);
    // }

    for i in &xs {
        println!("{}", i);
    }
}
