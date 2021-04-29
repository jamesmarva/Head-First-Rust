use std::ascii::AsciiExt;

fn main() {
    let mut dt = vec!['a', 'b', 'c', 'd'];
    {
        let slice = &mut dt[..];
        upper_all(slice);
    }
    dt.push('e');
    dt.push('f');

    let dt1 = foo();

}

fn upper_all(dt: &mut [char]) {
    for c in dt {
        c.make_ascii_uppercase();
    }
}

fn foo() -> Vec<char> {
    let mut dt = vec!['a', 'b', 'c', 'd'];
    let slice = &mut dt[..];
    upper_all(slice);
    dt.push('e');
    dt
}
