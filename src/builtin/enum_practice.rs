enum Discount {
    Flat(f32),
    Percent(f32)
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f32   
}

impl Product {
    fn apply_discount(&mut self,  discount: &Discount) {
        match discount {
            Discount::Flat(amount) => self.price = self.price - amount,
            Discount::Percent(amount) => {
                let discount_price =  self.price * (amount / 100.0);
                self.price = self.price - discount_price;
            },
        };
    }
}

pub fn practice(){
    let mut t_shirt = Product{name: "T-Shirt".to_owned(), price: 15000.0};
    let flat = Discount::Flat(3000.0);

    println!("before applying discount : {:?}", t_shirt);
    t_shirt.apply_discount(&flat);
    println!("after applying discount : {:?}", t_shirt);

    let mut black_jacket = Product{name: String::from("Black-Jacket"), price: 86000.0};
    let percent = Discount::Percent(50.0);

    println!("before applying discount : {:?}", black_jacket);
    black_jacket.apply_discount(&percent);
    println!("after applying discount : {:?}", black_jacket);
}