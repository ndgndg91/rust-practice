/// GroceryStore has Grocery Items.
struct GroceryStore {
    grocery_items: Vec<GroceryItem>,
}

/// GroceryItem has name, stock, price.
#[allow(dead_code)]
struct GroceryItem {
    name: String,
    stock: i32,
    price: f64,
}

/// GroceryStore has Grocery Items.
impl GroceryStore {
    fn find_stock_by_name(&self, name: &str) -> Option<i32> {
        for item in &self.grocery_items {
            if item.name == name {
                return Some(item.stock);
            }
        }
        None
    }
}

pub fn practice() {
    let store = GroceryStore{ grocery_items: vec![
        GroceryItem {
            name: "Banana-Milk".to_owned(),
            stock: 10,
            price: 2500.0,
        },
        GroceryItem{
            name: "Merona".to_owned(),
            stock: 30,
            price: 1500.0
        },
        GroceryItem{
            name: "My-Gumi".to_owned(),
            stock: 15,
            price: 1000.0
        }
    ]};
    
    let merona_stock = store.find_stock_by_name("Merona");
    println!("Merona stock : {:?}", merona_stock);
    let none = store.find_stock_by_name("not-exists");
    println!("none : {:?}", none);
}