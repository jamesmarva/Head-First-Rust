use std::rc::Rc;

fn main() {

    struct Node {
        nt: Option<Rc<Node>>
    }
    
    impl Node {
        fn new() -> Node {
            Node {nt : None}
        }
    }

    impl Drop for Node {

        fn drop(&mut self) {
            println!("drop node");
        }
    }


    let mut n1 = Rc::new(Node {nt: None});
    let mut n2 = Rc::new(Node {nt: None});
    let mut s3 = Rc::new(Node {nt: None});

    n1.nt = Some(n2);
    n2.nt = Some(n3);
    n3.nt = Some(n1);
}
