use std::rc::Rc;

struct Node {
    next: Option<Rc<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop");
    }
}


fn main() {
    
    let mut n1 = Node {next: None};

    let mut n2 = Node {next: None};

    let mut n3 = Node {next: None};

    n1.next = Some(Rc::new(n2));

    n2.next = Some(Rc::new(n3));
    n3.next = Some(Rc::new(n1));
    
}

// main.rs(18, 9): move occurs because `n2` has type `Node`, 
// which does not implement the `Copy` trait
// main.rs(22, 28): value moved here
