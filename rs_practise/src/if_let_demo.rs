pub fn if_let_demo() {
    let maybe_user = Some("user");
    if let Some(user) = maybe_user {
        println!("user: {}", user);
    }

    let maybe_user = Some("user");
    if let Some(user) = maybe_user {
        println!("user: {}", user);
    } else {
        println!("no user");
    }
}
