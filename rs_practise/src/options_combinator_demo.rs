pub fn options_combinator_demo() {
    let a = Some(1);
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    println!("a_is_some: {:?}, a_is_none: {:?}", a_is_some, a_is_none);
    let a_mapped = a.map(|x| x + 1);
    println!("a_mapped: {:?}", a_mapped);
    let a_filtered = a.filter(|x| x == &1);
    println!("a_filtered: {:?}", a_filtered);
    let a_or_else = a.or_else(|| Some(2));
    println!("a_or_else: {:?}", a_or_else);
    let a_unwrap = a.unwrap_or_else(|| 2);
    println!("a_unwrap: {:?}", a_unwrap);
}
