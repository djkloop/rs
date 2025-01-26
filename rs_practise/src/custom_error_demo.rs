use thiserror::Error;

#[derive(Error, Debug)]
enum ProgramError {
    #[error("menu error")]
    MenuError(#[from] MenuError),
    #[error("math error")]
    MathError(#[from] MathError),
}

#[derive(Error, Debug)]
enum MenuError {
    #[error("Menu item not found")]
    NotFound,
}

#[derive(Error, Debug)]
enum MathError {
    #[error("Division by zero")]
    DivisionByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn run(setup: i32) -> Result<(), ProgramError   > {
    if setup == 1 {
        pick_menu("4")?;
    } else if setup == 2 {
        divide(1, 0)?;
    }
    Ok(())
}

pub fn custom_error_demo() {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
}
