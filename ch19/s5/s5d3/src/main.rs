fn main() {


}

fn test<'a>(s: Box<&'static str>) {
    let local : Box<&'a str> = s;
}
