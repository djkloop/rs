enum Ticked {
    Standard(f64, String),
    Backstage(f64),
    Vip(f64, String),
}

pub fn advance_match_demo() {
    let tickets = vec![
        Ticked::Standard(50.0, "Standard".to_owned()),
        Ticked::Backstage(100.0),
        Ticked::Vip(150.0, "Vip".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticked::Standard(price, event) => {
                println!("Standard ticket for {} is {}", event, price)
            }
            Ticked::Backstage(price) => println!("Backstage ticket is {}", price),
            Ticked::Vip(price, event) => println!("Vip ticket for {} is {}", event, price),
        }
    }
}
