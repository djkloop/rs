enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

pub fn type_annotations_demo() {
    let flat = Discount::Flat(5);
    match flat {
        Discount::Flat(5) => println!("Flat discount of 5"),
        Discount::Flat(amount) => println!("Flat discount of {}", amount),
        _ => println!("No discount"),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        Ticket { price: 50, event } => println!("event @ 50 = price is {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
