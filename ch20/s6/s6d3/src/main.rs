fn main() {
    let mut ori = vec!["0".to_string(), String::from("1"), String::from("2"), String::from("3"), String::from("4")];

    {
        let mut d = ori.drain(1..3);
        let s : Option<String> = d.next();
        println!("{:?}", s);

        let s : Option<String> = d.next();
        println!("{:?}", s);

        let s : Option<String> = d.next();
        println!("{:?}", s);

        std::mem::forget(d);
    }

    println!("left:");
    for i in ori.iter() {
        println!("{:?}", i);
    }

    let mut ori = vec!["0".to_string(), String::from("1"), String::from("2"), String::from("3"), String::from("4")];
    ori.drain(1..3);
    for i in ori {
        println!("{}", i);
    }
}
