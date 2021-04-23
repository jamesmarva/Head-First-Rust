fn main() {

}

/// 注意：以下两个 trait 的的函数声明都是有效的，而且声明的都是成员方法
/// 
/// 
trait T {
    fn m1(self: Self);
    fn m2(self: &Self);
    fn m3(self: &mut Self);
}

trait R {
    fn m1(self);
    fn m2(&self);
    fn m3(&mut self);
}
