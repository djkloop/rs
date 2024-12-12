use std::fs;

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn main() {
    let result = read_file("assets/x1.csv");
    match result {
        Ok(content) => println!("{}", content),
        Err(err) => println!("{:?}", err.to_string()),
    }
}
