
#[derive(Debug)]
enum Direction {
    East, West, South, North,
}

fn direction_to_int(d: Direction) -> i32 {
    match d {
        Direction::East => 10,
        Direction::West => 20,
        Direction::South => 30,
        Direction::North => 40,
    }
}

fn main() {
    let d = Direction::East;
    let rst = direction_to_int(d);
    print!("rst: {}", rst);


    let x = 1;
    let r = match x {
        -1 => "negative".to_string(),
        0 => "zero".to_string(),
        1 => "positive".to_string(),
        _ => "error".to_string(),
    };
    println!("{}", r);
}
