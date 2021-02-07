extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter};

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
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}