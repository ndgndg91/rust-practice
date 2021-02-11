mod beverage;
mod employee;

pub fn practice() {
    just_loop();
    just_while();

    just_matches();
    enum_with_match();
    grocery_item();
    beverage::practice();

    just_tuple();

    print_string("a string slice");
    let borrow_string = "borrow string";
    print_string(borrow_string);
    print_string(borrow_string);
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_string(&owned_string);
    print_string(&another_owned);

    employee::employee();
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


fn just_tuple(){
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let user_info = ("Dong-Gil", 31);
    println!("{:?}, {:?}", user_info.0, user_info.1);

    let favorites = (
        "Game",
        "Dev",
        "rust",
        "java",
        "go",
        "spring",
        "js",
        "typescript",
    );
    println!(
        "{:?}, {:?}, {:?} {:?}",
        favorites.0, favorites.2, favorites.4, favorites.7
    );
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn just_matches() {
    just_match(true);
    just_match(false);

    just_match_integer();
    just_match_string();
}

fn enum_with_match() {
    // enum_with_match function has a go variable ownership.
    let go = Direction::UP;
    // which_way function take a go varialbe ownership. so, enum_with_match function loose a go variable ownership.
    which_way(go);
    // so, below code raise error.
    // which_way(go);
    which_way(Direction::DOWN);
    which_way(Direction::LEFT);
    which_way(Direction::RIGHT);
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

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn grocery_item() {
    let item = GroceryItem {
        stock: 10,
        price: 9500.0,
    };
    println!("price : {:?}, stock : {:?}", item.price, item.stock);
}
