const MOCK_DATA: &'static str = include_str!("./mock_data.csv");

#[derive(Debug)]
struct Names<'a> {
    inner: Vec<&'a str>,
}

#[derive(Debug)]
struct Titles<'a> {
    inner: Vec<&'a str>,
}

pub fn lifetimes_structures_a() {
    let data: Vec<_> = MOCK_DATA.split("\n").skip(1).collect();
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(",").nth(1))
        .collect();
    let names = Names { inner: names };

    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(",").nth(4))
        .collect();
    let titles = Titles { inner: titles };

    let data = names.inner.iter().zip(titles.inner.iter());
    println!("{:?}", data);
    for (name, title) in data {
        println!("name: {}, title: {}", name, title);
    }
}
