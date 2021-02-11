mod beverage;
mod employee;
mod string_practice;
mod repeat_practice;
mod tuple_practice;
mod enum_with_match;
mod match_practice;
mod grocery_item;

pub fn practice() {
    repeat_practice::repeat_practice();
    match_practice::practice();
    enum_with_match::practice();
    tuple_practice::tuple_practice();
    string_practice::string_practice();
    employee::employee();
    beverage::practice();
    grocery_item::practice();
}
