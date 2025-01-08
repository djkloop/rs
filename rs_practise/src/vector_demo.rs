struct Test {
    score: i32,
}

pub fn vector_demo() {
    let scores = vec![Test { score: 10 }, Test { score: 20 }, Test { score: 30 }];
    for score in scores {
        println!("score = {:?}", score.score);
    }
}
