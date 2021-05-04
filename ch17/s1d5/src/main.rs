use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    alloc_objects();
    println!("finish.");
}

struct Node {
    nt: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new() -> Node {
        Node {nt: None}
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn alloc_objects() {
    let n1 = Rc::new(RefCell::new(Node::new()));
    let n2 = Rc::new(RefCell::new(Node::new()));
    let n3 = Rc::new(RefCell::new(Node::new()));

    n1.borrow_mut().nt = Some(n2.clone());
    n2.borrow_mut().nt = Some(n3.clone());
    n3.borrow_mut().nt = Some(n1.clone());
}
// 1 使用引用计数指针
// 2 存在内部可变性
// 3 指针指向的内容的本身不是'static 


