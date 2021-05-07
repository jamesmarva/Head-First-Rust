fn main() {
    // test_ret(s)
}

fn s(arg: i32) -> &'static str {
    return "";
}

fn test_ret<'a>(f: fn() -> &'a str) {
    f();
}


// 
fn test_ret1<'a>(f : fn()->&'static str) {
    let local : fn()->&'a str = f;
}

fn test_ret2<'a>(f : fn() -> &'static str) {
    let local : fn() -> &'a str = f;
}

