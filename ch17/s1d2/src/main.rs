struct Node {
    next: Option<Box<Node>>,
}

fn main() {

    let mut n1 = Box::new(Node {next: None});

    let mut n2 = Box::new(Node {next: None});

    n1.next = Some(n2);

    match n1.next {
        Some(mut n) => n.next = Some(n1),
        None => {}
    }


}
// main.rs(14, 14): value moved here