fn maybe_num() -> Option<i32> {
    Some(42)
}

fn maybe_word() -> Option<String> {
    Some("Hello".to_string())
}

pub fn map_combinator_demo() {
    let plus_one = match maybe_num() {
        Some(n) => Some(n + 1),
        None => None,
    };
    println!("{:?}", plus_one);
    let plus_one = maybe_num().map(|n| n + 1);
    println!("{:?}", plus_one);

    let word_length = maybe_word().map(|w| w.len());
    println!("{:?}", word_length);

    let word_length_double = maybe_word().map(|w| w.len()).map(|len| len * 2);
    println!("{:?}", word_length_double);
}