pub fn string_practice() {
    print_string("a string slice");
    let borrow_string = "borrow string";
    print_string(borrow_string);
    print_string(borrow_string);
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_string(&owned_string);
    print_string(&another_owned);
}

fn print_string(text: &str) {
    println!("{:?}", text);
}
