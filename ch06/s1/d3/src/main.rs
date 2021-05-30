fn main() {
    let v = [[1i32; 5]; 5];
    for i in &v {
        println!("{:?}", i);
    }

    let mut v1 = [1i32; 4];
    let mut_ref_v1 = &mut v1;
    mut_ref_v1[1] = 1;
    mut_ref_v1[0] = 222;
    // mut_ref_v1[100] = 1111;

    let v2 = [1u32; 6];
    let ref_v2 = &v2;
    println!("{}", ref_v2[333]); // note: 这里会报错
}
