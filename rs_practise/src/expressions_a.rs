

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("to big"),
        false => println!("to small"),
    }   
}

pub fn expressions_a() {
    let val = 100;
    let is_gt_100 = val > 100;
    print_message(is_gt_100);
}