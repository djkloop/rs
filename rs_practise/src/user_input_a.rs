use std::io;

#[derive(Debug)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,  
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state_lower = state.trim().to_lowercase();
        match state_lower.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_state(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Off"),
        Sleep => println!("Sleep"),
        Reboot => println!("Reboot"),
        Shutdown => println!("Shutdown"),
        Hibernate => println!("Hibernate"),
    }
}

pub fn user_input_a() {
    let mut input = String::new();
    println!("Enter a power state: ");
    io::stdin().read_line(&mut input).unwrap();
    let power_state = PowerState::new(&input);
    if let Some(state) = power_state {
        print_power_state(state);
    } else {
        println!("Invalid power state entered.");
    }
}

