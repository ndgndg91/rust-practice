mod math;
mod builtin;
mod my;

// This is Entry point of the applicaiton. 
// like public static void main(String[] args) {} in Java.
fn main() {
    let result_1 = math::add(1, 9);
    let result_2 = math::add(10, 90);
    let result_3 = math::add(result_2, 900);

    println!("1 + 9 = {:?}", result_1);
    println!("10 + 90 = {:?}", result_2);
    println!("result_2 + 900 = {:?}", result_3);
    
    println!("100 is big number : {:?}", math::big_number(100));
    println!("50 is big number : {:?}", math::big_number(50));

    builtin::just_loop();
    builtin::just_while();

    math::arithmetic();
    builtin::just_match(true);
    builtin::just_match(false);

    builtin::just_match_integer();
    builtin::just_match_string();

    // main function has a go variable ownership.
    let go = builtin::Direction::UP;
    // which_way function take a go varialbe ownership. so, main function loose a go variable ownership.
    builtin::which_way(go);
    // so, below code raise error.
    // which_way(go); 
    builtin::which_way(builtin::Direction::DOWN);
    builtin::which_way(builtin::Direction::LEFT);
    builtin::which_way(builtin::Direction::RIGHT);

    builtin::grocery_item();
    builtin::beverage();

    let coord = (2,3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2,3);
    println!("{:?}, {:?}", x, y);

    let user_info = ("Dong-Gil", 31);
    println!("{:?}, {:?}", user_info.0, user_info.1);

    let favorites = ("Game", "Dev", "rust", "java", "go", "spring", "js", "typescript");
    println!("{:?}, {:?}, {:?} {:?}", favorites.0, favorites.2, favorites.4, favorites.7);

    let role = builtin::Role::GUEST;
    // admin only allowed.
    let can_access_file = match role {
        builtin::Role::ADMIN => true,
        _ => false
    };

    println!("can access ? {:?}", can_access_file);
    let beverages = vec![builtin::Beverage::apple(), builtin::Beverage::remon()];
    for b in &beverages {
        b.print_self();
    }

    println!("beverage length : {:?}", beverages.len());

    // after for loop beverages not available by ownership
    for b in beverages {
        println!("fluid oz : {:?}", b.fluid_oz);
    }

    builtin::print_string("a string slice");
    let borrow_string = "borrow string";
    builtin::print_string(borrow_string);
    builtin::print_string(borrow_string);
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    builtin::print_string(&owned_string);
    builtin::print_string(&another_owned);

    let name = String::from("Name Dong Gil");
    let employee = builtin::Employee{name: name};
    println!("{:?}", &employee.name);
    println!("{:?}", &employee.name);
}