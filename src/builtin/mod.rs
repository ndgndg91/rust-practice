pub fn just_loop() {
    let mut mutate_variable = 0;
    loop {
        if mutate_variable > 10 {
            break;
        }

        println!("{:?}", mutate_variable);
        mutate_variable += 1;
    }
}

pub fn just_while() {
    let mut idx = 0;
    while idx < 10 {
        println!("idx : {:?}", idx);
        idx += 1;            
    }
}

pub fn print_string(text: &str) {
    println!("{:?}", text);
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub fn which_way(go: Direction) {
    match go {
        Direction::UP => println!("go up"),
        Direction::DOWN => println!("go down"),
        Direction::LEFT => println!("go left"),
        Direction::RIGHT => println!("go right"),
    }
}

pub fn just_match(v: bool) {
    match v {
        true => println!("just_match true!"),
        false => println!("just_match false!"),
    }
}

// match must handle all case. so _ use.
pub fn just_match_integer() {
    let value = 1;
    match value {
        1 => println!("value is 1"),
        2 => println!("vlaue is 2"),
        3 => println!("value is 3"),
        _ => println!("other case ignored"),
    }
}

// match must handle all case. so _ use.
pub fn just_match_string() {
    let my_name = "Nam Dong Gil";
    match my_name {
        "Bob" => println!("Not Dong Gil"),
        "Nam Dong Gil" => println!("Hi Giri"),
        _ => println!("Ignore other case!"),
    }
}

pub enum Role {
    MANAGER,
    ADMIN,
    USER,
    GUEST
}

// Debug to use "{:?}" token
// Clone, Copy not to use onwership, borrow, so always copy variable
// Clone, Copy is so expensive. so carafully use Clone, Copy Struct that have 3 ~ 4 entries
#[derive(Debug, Clone, Copy)]
enum Flavor {
    ORANGE,
    REMON,
    APPLE
}
#[derive(Debug, Clone, Copy)]
pub struct Beverage {
    flavor: Flavor,
    pub fluid_oz: f64
}

impl Beverage {
    pub fn apple() -> Self {
        Self {flavor: Flavor::APPLE, fluid_oz: 30.5}
    }
    pub fn remon() -> Self {
        Self {flavor: Flavor::REMON, fluid_oz: 35.5}
    }
    pub fn print_self(&self) {
        let description = match &self.flavor {
            Flavor::ORANGE => "ORANGE",
            Flavor::REMON => "REMON",
            Flavor::APPLE => "APPLE"
        };
        println!("{:?}, {:?}", description, self.fluid_oz)
    } 
}

pub struct Employee {
    pub name: String,
}

pub struct GroceryItem {
    stock: i32,
    price: f64,
}

pub fn beverage() {
    let apple_bever = Beverage::apple();
    apple_bever.print_self();
    print_flavor(&apple_bever.flavor);
    let remon_bever = Beverage::remon();
    remon_bever.print_self();
    let apple_beverage = Beverage{flavor: Flavor::APPLE, fluid_oz: 39.1};
    println!("{:?}", apple_beverage);
    // apple_beverage.print_self();
    let remon_beverage = Beverage{flavor: Flavor::REMON, fluid_oz: 31.9};
    println!("{:?}", remon_beverage);
    // remon_beverage.print_self();
    let orange_beverage = Beverage{flavor: Flavor::ORANGE, fluid_oz: 32.7};
    println!("{:?}", orange_beverage);
    // orange_beverage.print_self();

}

fn print_flavor(f: &Flavor) -> &'static str {
    return match f {
        Flavor::APPLE => "apple",
        Flavor::REMON => "remon",
        Flavor::ORANGE => "orange"
    }
}

pub fn grocery_item(){
    let item = GroceryItem{stock: 10, price: 9500.0};
    println!("price : {:?}, stock : {:?}", item.price, item.stock);
}