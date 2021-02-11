pub fn practice(){
    just_loop();
    just_while();

    just_match(true);
    just_match(false);

    just_match_integer();
    just_match_string();

    // main function has a go variable ownership.
    let go = Direction::UP;
    // which_way function take a go varialbe ownership. so, main function loose a go variable ownership.
    which_way(go);
    // so, below code raise error.
    // which_way(go); 
    which_way(Direction::DOWN);
    which_way(Direction::LEFT);
    which_way(Direction::RIGHT);

    grocery_item();
    beverage();

    let coord = (2,3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2,3);
    println!("{:?}, {:?}", x, y);

    let user_info = ("Dong-Gil", 31);
    println!("{:?}, {:?}", user_info.0, user_info.1);

    let favorites = ("Game", "Dev", "rust", "java", "go", "spring", "js", "typescript");
    println!("{:?}, {:?}, {:?} {:?}", favorites.0, favorites.2, favorites.4, favorites.7);

    let role = Role::GUEST;
    // admin only allowed.
    let can_access_file = match role {
        Role::ADMIN => true,
        _ => false
    };

    println!("can access ? {:?}", can_access_file);
    let beverages = vec![Beverage::apple(), Beverage::remon()];
    for b in &beverages {
        b.print_self();
    }

    println!("beverage length : {:?}", beverages.len());

    // after for loop beverages not available by ownership
    for b in beverages {
        println!("fluid oz : {:?}", b.fluid_oz);
    }

    print_string("a string slice");
    let borrow_string = "borrow string";
    print_string(borrow_string);
    print_string(borrow_string);
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_string(&owned_string);
    print_string(&another_owned);

    let name = String::from("Name Dong Gil");
    let employee = Employee{name: name};
    println!("{:?}", &employee.name);
    println!("{:?}", &employee.name);
}

fn just_loop() {
    let mut mutate_variable = 0;
    loop {
        if mutate_variable > 10 {
            break;
        }

        println!("{:?}", mutate_variable);
        mutate_variable += 1;
    }
}

fn just_while() {
    let mut idx = 0;
    while idx < 10 {
        println!("idx : {:?}", idx);
        idx += 1;            
    }
}

fn print_string(text: &str) {
    println!("{:?}", text);
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn which_way(go: Direction) {
    match go {
        Direction::UP => println!("go up"),
        Direction::DOWN => println!("go down"),
        Direction::LEFT => println!("go left"),
        Direction::RIGHT => println!("go right"),
    }
}

fn just_match(v: bool) {
    match v {
        true => println!("just_match true!"),
        false => println!("just_match false!"),
    }
}

// match must handle all case. so _ use.
fn just_match_integer() {
    let value = 1;
    match value {
        1 => println!("value is 1"),
        2 => println!("vlaue is 2"),
        3 => println!("value is 3"),
        _ => println!("other case ignored"),
    }
}

// match must handle all case. so _ use.
fn just_match_string() {
    let my_name = "Nam Dong Gil";
    match my_name {
        "Bob" => println!("Not Dong Gil"),
        "Nam Dong Gil" => println!("Hi Giri"),
        _ => println!("Ignore other case!"),
    }
}

#[allow(dead_code)]
enum Role {
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
struct Beverage {
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

struct Employee {
    pub name: String,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn beverage() {
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

fn grocery_item(){
    let item = GroceryItem{stock: 10, price: 9500.0};
    println!("price : {:?}, stock : {:?}", item.price, item.stock);
}