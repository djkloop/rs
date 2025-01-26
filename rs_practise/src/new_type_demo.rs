#[derive(Debug, Clone)]
struct NeverZero(i32);

impl NeverZero {
    fn new(value: i32) -> Result<Self, String> {
        if value == 0 {
            Err("Value cannot be zero".to_string())
        } else {
            Ok(Self(value))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    a / b.0
}

pub fn new_type_demo() {
    match NeverZero::new(0) {
        Ok(never_zero) => println!("{:?}", divide(10, never_zero)),
        Err(e) => println!("Error: {:?}", e),
    }
}
