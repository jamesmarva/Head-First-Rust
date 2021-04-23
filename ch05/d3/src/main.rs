struct Circle {
    radius: f64,
}

impl Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

fn main() {
    let c = Circle { radius: 3f64};
    println!("{}", c.get_radius());
}
