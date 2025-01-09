#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    match name {
        "John" => Some(1),
        "Jane" => Some(888),
        _ => None,
    }
}

pub fn map_combinator_a() {
    let user_name_john = "John".to_string();
    let user_john = find_user(&user_name_john).map(|id| User {
        user_id: id,
        name: user_name_john.to_owned(),
    });
    match user_john {
        Some(user) => println!("user_id: {:?}, name: {:?}", user.user_id, user.name),
        None => println!("User not found"),
    }

    let user_name_jane = "Jane".to_string();
    let user_jane = find_user(&user_name_jane).map(|id| User {
        user_id: id,
        name: user_name_jane.to_owned(),
    });
    match user_jane {
        Some(user) => println!("user_id: {:?}, name: {:?}", user.user_id, user.name),
        None => println!("User not found"),
    }

    let user_name_alice = "Alice".to_string();
    let user_alice = find_user(&user_name_alice).map(|id| User {
        user_id: id,
        name: user_name_alice.to_owned(),
    });
    match user_alice {
        Some(user) => println!("user_id: {:?}, name: {:?}", user.user_id, user.name),
        None => println!("User not found"),
    }
}
