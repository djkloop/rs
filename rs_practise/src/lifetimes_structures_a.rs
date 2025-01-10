const MOCK_DATA: &'static str = include_str!("./mock_data.csv");

pub fn lifetimes_structures_a() {
    let data: Vec<&str> = MOCK_DATA.split("\n").skip(1).collect();
    let names: Vec<&str> = data
        .iter()
        .filter_map(|line| line.split(",").nth(1))
        .collect();
    println!("names: {:?}", names);

    let titles: Vec<&str> = data
        .iter()
        .map(|line| line.replace("\r", ""))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            parts.get(4).copied()
        })
        .collect();
    println!("titles: {:?}", titles);
}
