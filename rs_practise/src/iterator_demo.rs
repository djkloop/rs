pub fn iterator_demo() {
    let v = vec![1, 2, 3, 4, 5];
    let mut plus_one = vec![];
    for i in v.iter() {
        plus_one.push(i + 1);
    }
    println!("plus_one: {:?}", plus_one);

    let plus_one: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("plus_one: {:?}", plus_one);

    let filter_nums: Vec<_> = v.iter().filter(|x| x <= &&3 ).collect();
    println!("filter_nums: {:?}", filter_nums);

    let find_num = v.iter().find(|x| x == &&3);
    println!("find_num: {:?}", find_num);

    let min_num = v.iter().min();
    println!("min_num: {:?}", min_num);

    let max_num = v.iter().max();
    println!("max_num: {:?}", max_num);

    let take_num = v.iter().take(3).collect::<Vec<_>>();
    println!("take_num: {:?}", take_num);

    let skip_num = v.iter().skip(2).collect::<Vec<_>>();
    println!("skip_num: {:?}", skip_num);
}
