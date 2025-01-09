fn add_function(a: i32, b: i32) -> i32 {
    a + b
}

pub fn closures_demo() {
    let sum = add_function(1, 2);
    println!("Sum: {}", sum);
    let sum2 = |a, b| a + b;
    println!("Sum2: {}", sum2(1, 2));
}