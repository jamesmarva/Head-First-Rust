fn main() {

    let c = Chef;
    // c.start(); wrong  multiple applicable items in scope
    <Chef as Cook>::start(&c);
    <Chef as Wash>::start(&c);
}


trait Cook {
    fn start(self: &Self);
}

trait Wash {
    fn start(self: &Self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash");
    }
}
