#[derive(Debug)]
enum Access {
    Admin,
    User,
    Guest,
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "user" => Some(Access::User),
        _ => None,
    }
}

fn part_1() -> bool {
    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
    maybe_access("root").or_else(root)
}

fn part_3() -> Access {
    maybe_access("alice").unwrap_or_else(|| Access::Guest)
}

pub fn options_combinator_a() {
    let part_1 = part_1();
    let part_2 = part_2();
    let part_3 = part_3();
    println!("part_1: {:?}", part_1);
    println!("part_2: {:?}", part_2);
    println!("part_3: {:?}", part_3);
}

