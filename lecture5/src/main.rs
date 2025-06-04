#[derive(PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

// enum TrafficLight {
//     Red,
//     Amber,
//     Green,
// }

// enum Result {
//     Ok,
//     Err,
// }
fn move_player(dir: Direction){
     match dir {
        Direction::North => println!("Moved to North"),
        Direction::South=> println!("Moved to South"),
        Direction::East => println!("Moved to East"),
        Direction::West => println!("Moved to West"),
     }
}

fn main() {
    let dir = Direction::West;
    move_player(dir);
}