fn main() {
    let v = vec![1, 2, 3, 4, 5];
    {
        // copy
        let cneter = v[2];
        println!("{}", cneter);
    }
    println!("{:?}", v);
}
