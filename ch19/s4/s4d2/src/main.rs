

fn main() {
    let mut x = [1i32, 2, 3];
    let pa = &mut x[0];
    // let pb = &mut x[1];
    // let pc = &x[2];
    
    *pa += 1;
    // let pc2 = &x[2];
    // *pa += 10;
    // println!("{}, {}, {}, {}", pa,  pb, pc, pc2);
    // println!("{}, {}, {}", pa,  pb, pc);
    // println!("{}, {}", pa, pc);
    println!("{} ", pa);
}
