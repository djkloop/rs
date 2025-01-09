mod sub {
    use std::collections::HashMap;

    pub fn sub_demo() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        println!("sub_demo: {:?}", map);
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

use sub::*;
use math::{add, sub};

pub fn modules_demo() {
    sub_demo();
    println!("add: {}", add(1, 2));
    println!("sub: {}", sub(1, 2));
}

