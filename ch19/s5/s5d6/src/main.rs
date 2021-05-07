use std::cell::Cell;

fn main() {
    
}

fn test<'a>(s: Cell<&'static str>) {
    let local : Cell<&'a str> = s;
}
