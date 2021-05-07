fn main() {

}


fn test_arg<'a>(f: fn(&'a str)) {
    let local : fn(&'static str) = f;
}

fn test_ret<'a>(f: fn()->&'a str) {
    let local : fn() -> &'static str = f;
}
