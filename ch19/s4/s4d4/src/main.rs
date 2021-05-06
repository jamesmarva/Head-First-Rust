fn main() {
    let mut x = [1i32, 2, 3];

    {
        let (first, rest): (&mut [i32], &mut [i32]) = x.split_at_mut(1);

        let (second, third): (&mut [i32], &mut [i32]) = rest.split_at_mut(1);

        first[0] += 2;

        second[0] += 33;

        third[0] += 44;

        println!("{:?}, {:?}, {:?}", first, second, third);
    }

    let x_slice = x.split_at_mut(0);
    println!("{:?}", x_slice);
    // println!("{:?}", x);
}
