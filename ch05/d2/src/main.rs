trait Shape {
    fn area(self: &Self) -> f64;
}


struct Circle {
    radius: f64,
}

impl Shape for Circle {

    fn area(self: &Self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}


fn main() {
    let c = Circle { radius: 2f64};

    println!("The area is {}", c.area());
}
