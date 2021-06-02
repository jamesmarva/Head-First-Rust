fn main() {
    let x = 1;

    let r = match x {
        -1 | 1 => "true",
        0 => "false",
        _ => "error",
    };
    println!("{}", r);

    let x = 'X';
    let r = match x {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        _ => "something else",
    };
    println!("{}", r);
}
