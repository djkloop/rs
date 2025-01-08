struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity = {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("id = {:?}", item.id);
}

pub fn ownership_a() {
    let item = GroceryItem {
        quantity: 10,
        id: 1,
    };
    display_quantity(&item);
    display_id(&item);
}