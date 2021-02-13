pub fn practice() {
    for num in 1..=5 {
        print!("{:?} \t", num);
    }
    println!("include 5, so 1 ~ 5");

    for num in 1..5 {
        print!("{:?} \t", num);
    }
    println!("not include 5, so 1 ~ 4");

    for alpha in 'a'..'f' {
        print!("{:?} \t", alpha);
    }
    println!("not include f, so a ~ e");

    for alpha in 'a'..='f' {
        print!("{:?} \t", alpha);
    }
    println!("include f, so a ~ f");
}