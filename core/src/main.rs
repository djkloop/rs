enum Direction {
    Left,
    Right,
    Up,
    Down,
}


fn main() {
    let go_direction_left = Direction::Left;
    match go_direction_left {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),
        Direction::Down => println!("go down"),
    }
}
