fn main() {
    println!("Hello, world!");
}


// 由于＆ ’ static str  <:  &’ a  str 以及 ＆’ a &'static  str  <:  &'a  &’ a  str 
// 关系成立， 这说明引用类型针对泛型参数 T 也是具备协变关系 的 
fn test<'a>(s: &'a &'static str) {
    let local: &'a &'a str = s;
}


fn test1<'a>(s: &'a mut &'static str) {
    let local : &'a mut &'a str = s;
}


