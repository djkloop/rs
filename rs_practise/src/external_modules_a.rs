use rs_practise::math::{add, mul, sub};

pub fn external_modules_a() {
    let result = {
        let two_plus_two = add(2, 2);
        let three = sub(two_plus_two, 1);
        mul(three, three)
    };
    println!("result: {}", result);

    {
        use rs_practise::msg::{capitalize, exciting, trim};
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
