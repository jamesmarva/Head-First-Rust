
/// value: 11, 11
/// address: 0x1d7d5e122c0, 0x1d7d5e122c0
fn main() {
    use std::rc::Rc;

    struct SharedValue {
        value: i32,
    }

    let shared_value : Rc<SharedValue> = Rc::new(SharedValue {value: 11});

    let owner1 = shared_value.clone();
    let owner2 = shared_value.clone();

    println!("value: {}, {}", owner1.value, owner2.value);

    println!("address: {:p}, {:p}", &owner1.value, &owner2.value);
}
