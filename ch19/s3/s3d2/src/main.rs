fn main() {
    fn swap<T>(x: &mut T, y: &mut T) {

        // 这里执行的move的操作，但是函数声明仅仅是可变借用，是不能进行move的
        let z: T = *x;
        // 如果允许执行这样的操作，会导致原来的借用指针x指向非法数据。
        *x = *y;
        *y = *x;
    }
}
