use std::collections::HashMap;

pub fn hashmap_a() {
    let mut map = HashMap::new();
    map.insert("Chair", 5);
    map.insert("Table", 10);
    map.insert("Bed", 15);
    map.insert("Sofa", 0);

    let mut total_stock = 0;
    for (key, value) in map.iter() {
        total_stock += value;
        let stock_count = if value == &0 {
            "Out of stock".to_owned()
        } else {
            format!("{:?} in stock", value)
        };
        println!("{}: {}", key, stock_count);
    }
    println!("Total stock: {}", total_stock);
}