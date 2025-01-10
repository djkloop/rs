use std::{collections::HashMap, io};

#[derive(Debug)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
}

pub fn get_input() -> Option<String> {
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

pub fn get_bill_amount() -> Option<f64> {
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::demo_bill::*;

    pub fn add_bill(bills: &mut Bills) {
        println!("Please enter the bill name: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        println!("Please enter the bill amount: ");
        let amount = get_bill_amount().unwrap_or(0.0);
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added successfully");
    }

    pub fn remove_bill(bills: &mut Bills) {
        println!("--------------------------------");
        println!("Viewing all bills");
        for bill in bills.get_all() {
            println!();
            println!("name = {:?}， amount = ${:?}", bill.name, bill.amount);
            println!();
        }
        println!("--------------------------------");
        println!("Please enter the bill name to remove: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("Bill removed successfully");
        } else {
            println!("Bill not found");
        }
    }

    pub fn view_bills(bills: &Bills) {
        println!();
        println!("--------------------------------");
        println!("Viewing all bills");
        println!();
        let bills = &bills.get_all();
        for bill in bills {
            println!("name = {:?}， amount = ${:?}", bill.name, bill.amount);
        }
        // 合计
        let total = bills.iter().fold(0.0, |sum, bill| sum + bill.amount);
        println!("name = total,  amount = ${:?}", total);
        println!();
        println!("--------------------------------");
        println!();
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    Quit,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::Quit),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("Welcome to Bill Manager");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Quit manager (or press Ctrl+C to exit)");
        println!("");
        println!("Enter Selection: ");
    }
}

pub fn demo_bill() {
    let mut bills = Bills::new();
    let mut is_running = true;

    while is_running {
        MainMenu::show();
        let input = get_input().expect("Failed to read line");
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => {
                menu::add_bill(&mut bills);
            }
            Some(MainMenu::ViewBill) => {
                menu::view_bills(&bills);
            }
            Some(MainMenu::RemoveBill) => {
                menu::remove_bill(&mut bills);
            }
            Some(MainMenu::Quit) => {
                println!("Exiting...");
                is_running = false;
            }
            None => {
                println!("Invalid input");
            }
        }
    }
}
