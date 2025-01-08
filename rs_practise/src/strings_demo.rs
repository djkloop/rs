struct LineItem {
    name: String,
    count: i32,
}

fn print_item_name(name: &str) {
    println!("name = {:?}", name);
}

fn print_item_count(count: i32) {
    println!("count = {:?}", count);
}

pub fn strings_demo() {
    let receipt = vec![
        LineItem {
            name: String::from("Apple"),
            count: 5,
        },
        LineItem {
            name: "Banana".to_owned(),
            count: 6,
        },
    ];

    for item in receipt {
        print_item_name(&item.name);
        print_item_count(item.count);
    }
}
