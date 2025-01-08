struct  Student {
    name: String,
    locker: Option<i32>,
}

pub fn option_a() {
    let mary = Student {
        name: "Mary".to_owned(),
        locker: Some(101),
    };
    println!("student name: {:?}", mary.name);
    match mary.locker {
        Some(locker) => println!("student locker: {:?}", locker),
        None => println!("student locker: None"),
    }

}