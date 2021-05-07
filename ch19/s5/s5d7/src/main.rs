fn main() {

}

// *mut T针对T 参数是不变关系。
fn test_raw_mut<'a>(s : *mut &'static str) {
    let local : *mut &'a str = s;
}


// *const T针对T参数具备协变关系
fn test_raw_const<'a>(s : *const &'static str) {
    let local : *const &'static str = s;
}
