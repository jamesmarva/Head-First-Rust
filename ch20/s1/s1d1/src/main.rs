fn main() {
    let mut v1 = Vec::<i32>::new();

    println!("Start: length {} capacity {}", v1.len(), v1.capacity());

    for i in 1..10 {
        v1.push(i);
        println!("pushed {}; length: {}; capacity: {}", i, v1.len(), v1.capacity());
    }

    let mut v2 = Vec::<i32>::with_capacity(1);
    println!("Start: length {}; capacity: {}", v1.len(), v2.capacity());

    v2.reserve(10);

    for i in 1..10 {
        v2.push(i);
        println!("Push {}; length: {}; capacity: {};", i, v2.len(), v2.capacity())
    }
}
