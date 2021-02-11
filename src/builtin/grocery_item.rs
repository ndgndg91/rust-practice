struct GroceryItem {
    stock: i32,
    price: f64,
}

pub fn practice() {
    let item = GroceryItem {
        stock: 10,
        price: 9500.0,
    };
    println!("price : {:?}, stock : {:?}", item.price, item.stock);
}