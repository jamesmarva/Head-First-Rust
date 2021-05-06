fn main() {
    let x = vec![1, 2, 3, 4, 5];

    unsafe {
        let t: (usize, usize)= std::mem::transmute_copy(&x);
        println!("{}", t.0);

        let t1: (usize, usize, usize, usize, usize, usize)= std::mem::transmute_copy(&x);
        println!("{}, {}, {}", t1.0, t1.1, t1.2);
        println!("{}, {}, {},  {}, {}", t1.0, t1.1, t1.2, t1.3, t1.4);

        println!("{}, {}, {},  {}, {}, {}", t1.0, t1.1, t1.2, t1.3, t1.4, t1.5);
    }
       
}
