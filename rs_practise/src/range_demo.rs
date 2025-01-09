pub fn range_demo() {
    let range = 1..=3;
    for i in range {
        println!("i: {}", i);
    }

    let w = 'a'..='z';
    for i in w {
        println!("i: {}", i);
    }
}
