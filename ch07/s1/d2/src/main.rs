
enum Direction {
    East, 
    West,
    South, 
    North
}

fn print(x: Direction) {
    match x {
        Direction::East => {
            println!("East");
        }
        Direction::West => {
            println!("West");
        }
        Direction::South => {
            println!("South");
        }
        Direction::North => {
            println!("North");
        }
        _ => {
            println!("other");
        }
    }
}

fn main() {
    let x = Direction::East;
    print(x);
}   

