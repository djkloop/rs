pub fn while_let_demo() {
    let mut data = Some(1);
    while let Some(_) = data {
        data = None
    }
    println!("data: {:?}", data);

    let nums = vec![1, 2, 3, 4, 5];
    let mut iter = nums.iter();
    while let Some(n) = iter.next() {
        println!("n: {}", n);
    }
}
