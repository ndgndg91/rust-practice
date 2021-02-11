pub fn practice(){
    let result_1 = add(1, 9);
    let result_2 = add(10, 90);
    let result_3 = add(result_2, 900);

    println!("1 + 9 = {:?}", result_1);
    println!("10 + 90 = {:?}", result_2);
    println!("result_2 + 900 = {:?}", result_3);
    
    println!("100 is big number : {:?}", big_number(100));
    println!("50 is big number : {:?}", big_number(50));
    arithmetic();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn big_number(value: i32) -> bool {
    value > 99
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
