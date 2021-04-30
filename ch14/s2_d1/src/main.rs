use std::ascii::AsciiExt;


//
// 如果执行了push（）方法后，引
// 发了Vec数据结构的扩容，它把以前的空间释放掉，申请了新的空间，
// 进入下一轮循环的时候，slice就会指向一个非法地址，会出现内存不安
// 全。以上这段代码理应出现编译错误。
fn main() {

    let mut dt = vec!['a', 'b', 'c'];
    let slice = &mut dt[..];
    loop {
        capitalize(slice);
        break;
        // dt.push('d');
    }
    dt.push('e');

}

fn capitalize(dt: &mut [char]) {
    for c in dt {
        c.make_ascii_uppercase();
    }
}