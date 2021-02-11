pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn big_number(value: i32) -> bool {
    value > 99
}

pub fn arithmetic() {
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