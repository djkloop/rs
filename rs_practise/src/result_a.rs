#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 18 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("age must be greater than 18")
        }
    }
}

pub fn result_a() {
    let child = Adult::new(17, "XiaoMing");
    let adult = Adult::new(18, "John");
    match child {
        Ok(child) => println!("child: {:?}", child),
        Err(e) => println!("child error: {:?}", e),
    }
    match adult {
        Ok(adult) => println!("adult: {:?}", adult),
        Err(e) => println!("adult error: {:?}", e),
    }
}
