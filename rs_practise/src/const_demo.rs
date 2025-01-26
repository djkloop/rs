const MAX_SPEED: i32 = 9999;

fn clamped_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}

pub fn const_demo() {
    println!("Clamped speed: {}", clamped_speed(8888));
}
