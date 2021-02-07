extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter};

enum Flavor {
    ORANGE,
    REMON,
    APPLE
}

struct Beverage {
    flavor: Flavor,
    fluid_oz: f64
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn beverage() {
    let apple_beverage = Beverage{flavor: Flavor::APPLE, fluid_oz: 39.1};
    println!("flaver : {:?}, oz : {:>}", print_flavor(apple_beverage.flavor), apple_beverage.fluid_oz);
    let remon_beverage = Beverage{flavor: Flavor::REMON, fluid_oz: 31.9};
    println!("flaver : {:?}, oz : {:>}", print_flavor(remon_beverage.flavor), remon_beverage.fluid_oz);
    let orange_beverage = Beverage{flavor: Flavor::ORANGE, fluid_oz: 32.7};
    println!("flaver : {:?}, oz : {:>}", print_flavor(orange_beverage.flavor), orange_beverage.fluid_oz);
}

fn print_flavor(f: Flavor) -> &'static str {
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

// This is Entry point of the applicaiton. 
// like public static void main(String[] args) {} in Java.
fn main() {
    let out = b"Hello fellow Rustaceans!";
    let width = 32;

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();

    let result_1 = add(1, 9);
    let result_2 = add(10, 90);
    let result_3 = add(result_2, 900);

    println!("1 + 9 = {:?}", result_1);
    println!("10 + 90 = {:?}", result_2);
    println!("result_2 + 900 = {:?}", result_3);
    
    println!("100 is big number : {:?}", big_number(100));
    println!("50 is big number : {:?}", big_number(50));

    just_loop();
    just_while();

    arithmetic();
    just_match(true);
    just_match(false);

    just_match_integer();
    just_match_string();

    let go = Direction::UP;
    which_way(go);
    which_way(Direction::DOWN);
    which_way(Direction::LEFT);
    which_way(Direction::RIGHT);

    grocery_item();
    beverage();
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

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn big_number(value: i32) -> bool {
    value > 99
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

fn arithmetic() {
    let sum = 2 + 2;
    let value = 10 - 5;
    let division = 10 / 2;
    let multiply = 5 * 5;

    let two = sub(5, 3);
    let remain = 5 % 3;

    println!("sum : {:?}", sum);
    println!("value : {:?}", value);
    println!("division : {:?}", division);
    println!("multiply : {:?}", multiply);
    println!("two : {:?}", two);
    println!("remain : {:?}", remain);
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
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