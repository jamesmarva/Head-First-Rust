use std::ascii::AsciiExt;

fn main() {
    let mut data = vec!['a', 'b', 'c', 'd'];
    capitalize(&mut data);

    data.push('e');
    data.push('f');
}

fn capitalize(data: &mut [char]) {
    for c in data {
        c.make_ascii_uppercase();
    }
}
