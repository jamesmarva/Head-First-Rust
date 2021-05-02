use std::borrow::Cow;

fn main() {

    let s1 = "no_space_in_string";
    let rst1 = remove_spaces(s1);

    let s2 = "spaces in string";
    let rst2 = remove_spaces(s2);
    println!("{}", rst1);
    println!("{}", rst2);
}


fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }
        return Cow::Owned(buf);
    }
    return Cow::Borrowed(input);
}
