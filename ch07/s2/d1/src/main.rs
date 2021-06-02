fn main() {

    let d = Direction::East;

    match d {
        Direction::East => {
            print!("east");
        }
        Direction::West => {
            print!("West");
        }

        Direction::South => {
            print!("South");
        }

        Direction::North => {
            print!("North");
        }
    }
}

#[non_exhaustive]
enum Direction {
    East,
    West,
    South,
    North,
}
