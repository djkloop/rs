fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


pub fn lifetimes_functions_a() {
    let short = "short";
    let long = "long long long";
    println!("{}", longest(short, long));
}
