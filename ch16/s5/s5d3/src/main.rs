use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_vec : Rc<RefCell<Vec<isize>>> = Rc::new(RefCell::new(vec![1, 2, 3]));

    let share1 = shared_vec.clone();
    let share2 = share1.clone();

    share1.borrow_mut().push(4);
    println!("{:?}", shared_vec.borrow());

    share2.borrow_mut().push(5);
    println!("{:?}", shared_vec.borrow());

    share1.borrow_mut().push(6);
    println!("{:?}", shared_vec.borrow());
    
}
