
fn main() {
   let x = T2 {
       i1: T1(0, 'A'),
       i2: false,
   };

   let T2 {
       i1:T1(val1, val2),
       i2: val3,
   }  = x;
   println!("{}, {}, {}", val1, val2, val3);
}


struct T1 (i32, char);

struct T2 {
    i1: T1,
    i2: bool,
}
