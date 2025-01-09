use chrono::{DateTime, Local};

pub fn external_crates_demo() {
    let now: DateTime<Local> = Local::now();
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("now: {}", formatted);
}
