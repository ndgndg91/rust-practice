#[allow(dead_code)]
fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

pub fn practice(){
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };

    let add2 = |a, b| a + b;
    let sum = add(1,2);
    let sum2 = add2(1,2);

    println!("add(1,2) = {}, add2(1,2) = {}", sum, sum2);

}