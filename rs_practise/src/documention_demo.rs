/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - The first number to add.
/// * `b` - The second number to add.
///
/// # Returns
///
/// The sum of the two numbers.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn documention_demo() {
    println!("documentation demo");
    let result = add(1, 2);
    println!("result: {}", result);
}
