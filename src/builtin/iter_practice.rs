pub fn practice() {
    let numbers = vec![1, 2, 3, 4, 5];

    // mutate new vector allocation.
    let mut plus_one_numbers = vec![];
    for number in &numbers {
        plus_one_numbers.push(number + 1);
    }
    // use iter
    let plus_1_numbers: Vec<i32> = numbers.iter().map(|n| n + 1).collect();
    dbg!(plus_1_numbers);

    let multiply_2_numbers: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    dbg!(multiply_2_numbers);

    let lte_3_number: Vec<&i32> = numbers.iter().filter(|&&n| n <= 3).collect();
    dbg!(lte_3_number);

    let gte_2_number: Vec<&i32> = numbers.iter().filter(|&&n| n >= 2).collect();
    dbg!(gte_2_number);

    // chaining
    let big_numbers = vec![100, 200, 300, 400, 500, 600, 700, 800];

    let chaining: Vec<i32> = big_numbers
        .iter()
        .filter(|&&n| n <= 700)
        .map(|n| n * 2)
        .filter(|&n| n >= 300)
        .skip(2)
        .take(2)
        .collect();
    dbg!(chaining);
}
