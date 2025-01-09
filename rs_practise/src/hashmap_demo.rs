use std::collections::HashMap;

struct Contents {
    content: String
}

pub fn hashmap_demo() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: "Laptop".to_string() });
    lockers.insert(2, Contents { content: "Book".to_string() });
    lockers.insert(3, Contents { content: "Notebook".to_string() });

    for (key, value) in lockers.iter() {
        println!("Locker {} contains: {}", key, value.content);
    }
}
