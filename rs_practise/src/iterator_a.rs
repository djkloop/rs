pub fn iterator_a() {
    let data = vec![1, 2, 3, 4, 5].iter().map(|x| x + 1).filter(|x| x % 2 == 0).collect::<Vec<_>>();
    println!("data: {:?}", data);
}
