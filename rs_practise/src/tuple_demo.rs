
fn coordinates() -> (i32, i32) {
    (1, 8)
}

pub fn tuple_main() {
    let (x, y) = coordinates();
    if y > 5 {
        println!("The y coordinate is greater than 5");
    } else if y < 5 {
        println!("The y coordinate is less than 5");
    } else {
        println!("The y coordinate is equal to 5");
    }

}

