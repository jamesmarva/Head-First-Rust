struct Node {
    next: Option<Box<Node>>
}


fn main() {

    let mut n1 = Box::new(Node {next: None});

    let mut n2 = Box::new(Node {next: None});

    n1.next = Some(n2);
    // 这条语句
    // 中发生了move语义，从此句往后，node2变量的生命周期已经结束了。
    // 因为Some 里面传的是self

    n2.next = Some(n1);
}
