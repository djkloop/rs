#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Please select main, start or quit".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice: {:?}", choice);
}

fn pick_choice(input_choice: &str) -> Result<(), String> {
    let choice = get_choice(input_choice)?;
    print_choice(&choice);
    Ok(())
}

pub fn result_demo() {
    let choice_err = pick_choice("maixn");
    if !choice_err.is_ok() {
        println!("choice_err: {:?}", choice_err);
    }
}
