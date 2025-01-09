use std::io;

fn get_input() -> Option<String> {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).is_err() {
        println!("Please enter your data again.");
    }
    let input = input.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("Welcome to Bill Manager");
        println!("1. Add bill");
        println!("2. View bill");
        println!("");
        println!("Enter Selection: ");
    }
}

pub fn demo_bill() {

    loop {
        MainMenu::show();
        let input = get_input().expect("Failed to read line");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => (),
            Some(MainMenu::ViewBill) => (),
            None => return
        }
    }
}
