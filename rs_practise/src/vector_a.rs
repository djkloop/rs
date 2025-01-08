
pub fn vector_a() {
    let scores = vec![10, 20, 30];

    for score in &scores {
        match score {
            10 => println!("score = 10"),
            _ => println!("score = {:?}", score),
        }
    }
    println!("scores of elements len: {:?}", scores.len());
}
