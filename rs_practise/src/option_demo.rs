struct Survey { 
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

pub fn option_demo() {
    let response = Survey {
        q1: Some(10),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };


    match response.q1 {
        Some(answer) => println!("Q1: {:?}", answer), 
        None => println!("Q1: No answer"),
    }

    match response.q2 {
        Some(answer) => println!("Q2: {:?}", answer),
        None => println!("Q2: No answer"),
    }

    match response.q3 {
        Some(answer) => println!("Q3: {:?}", answer),
        None => println!("Q3: No answer"),
    }
}
