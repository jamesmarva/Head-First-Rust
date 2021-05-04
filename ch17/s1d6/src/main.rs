use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

fn main() {

    alloc_objects();
    println!("finish");
}

struct Node {
    nt: Option<Weak<RefCell<Node>>>
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop")
    }
}

impl Node {
    fn new() -> Self {
        Node {nt: None}
    }
}

fn alloc_objects() {
    let n1 = Rc::downgrade(&Rc::new(RefCell::new(Node::new())));
    let n2 = Rc::downgrade(&Rc::new(RefCell::new(Node::new())));
    let n3 = Rc::downgrade(&Rc::new(RefCell::new(Node::new())));

    let strong_n1: Option<Rc<RefCell<Node>>> = n1.upgrade();
    strong_n1.expect("n1 is None").borrow_mut().nt = Some(n2.clone());
    n2.upgrade().expect("n2 is None").borrow_mut().nt = Some(n3.clone());
    n3.upgrade().expect("n3 is None").borrow_mut().nt = Some(n1.clone());
}