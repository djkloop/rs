use std::io;

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

pub fn user_input_demo() {
    let mut all_inputs = vec![];
    let mut max_input_count = 3;

    while max_input_count > 0 {
        let input = get_input().unwrap();
        all_inputs.push(input);
        max_input_count -= 1;
    }
    println!("all_inputs: {:?}", all_inputs);
}