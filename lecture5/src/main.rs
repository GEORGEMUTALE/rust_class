// Define a simple enum
#![allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}
fn print_direction(dir: Direction){
match dir {
        Direction::North => println!("Heading north!"),
        Direction::South => println!("Heading south!"),
        Direction::East => println!("Heading east!"),
        Direction::West => println!("Heading west!"),
    }
}


fn main() {
    // Use the enum
    let my_direction = Direction::North;

    print_direction(my_direction);
    
}