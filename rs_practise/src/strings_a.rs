struct Person {
    name: String,
    favorite_color: String,
    age: i32,
}

impl Person {
    fn print_favorite_color(&self) {
        println!("color = {:?}", self.favorite_color);
    }
}

fn print_name(name: &str) {
    println!("name = {:?}", name);
}

pub fn strings_a() {
    let people = vec![
        Person {
            name: "Alice".to_owned(),
            favorite_color: "blue".to_owned(),
            age: 20,
        },
        Person {
            name: "Bob".to_owned(),
            favorite_color: "green".to_owned(),
            age: 30,
        },
        Person {
            name: "Charlie".to_owned(),
            favorite_color: "red".to_owned(),
            age: 40,
        },
    ];

    for person in people {
        if person.age <= 30 {
            person.print_favorite_color();
            print_name(&person.name);
        }
    }
}
