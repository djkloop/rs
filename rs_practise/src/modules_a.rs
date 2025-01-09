mod msg {
    pub fn trim(msg: &str) -> &str {
        msg.trim()
    }

    pub fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
        if let Some(letter) = msg.get(0..1) {
            format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).into()
        } else {
            msg.into()
        }
    }

    pub fn exciting(msg: &str) -> String {
        format!("{}!", msg)
    }
}

mod math {
    pub fn add(a: isize, b: isize) -> isize {
        a + b
    }

    pub fn sub(a: isize, b: isize) -> isize {
        a - b
    }

    pub fn mul(a: isize, b: isize) -> isize {
        a * b
    }
}

use math::{add, sub, mul};



pub fn modules_a() {
    let result = {
        let two_plus_two = add(2, 2);
        let three = sub(two_plus_two, 1);
        mul(three, three)
    };
    println!("result: {}", result);

    {
        use msg::*;
        let hello = {
            let msg = "hello ";
            let msg = trim(msg);
            capitalize(msg)
        };
    
        let world = {
            let msg = "world";
            let msg = trim(msg);
            exciting(msg)
        };

        let msg = format!("{}, {}", hello, world);
        println!("msg: {}", msg);
    }
    
}

