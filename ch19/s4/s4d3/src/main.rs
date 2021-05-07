fn main() {
    let mut x = [1i32, 2, 3];

    {
        let (first, rest) = x.split_at_mut(1);
        let (second, third) = rest.split_at_mut(1);
        first[0] += 2;
        second[0] += 4;
        third[0] += 8;
        println!("{:?} {:?} {:?}", first, second, third);
    }

    println!("{:?}", &x);
}
