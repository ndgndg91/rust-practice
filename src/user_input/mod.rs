use std::io;
mod exercise;

#[allow(dead_code)]
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    println!("user input status : {:?}", user_input_status);
    Ok(buffer.trim().to_owned())
}
#[allow(dead_code)]
pub fn practice(){
     let mut all_input = vec![];
     let mut times_input = 0;
     while times_input < 2 {
         match get_input() {
             Ok(words) => {
                 all_input.push(words);
                 times_input += 1;
             },
             Err(e) => println!("{:?}", e), 
         }
     }

     for input in all_input {
         println! ("Original : {:?}, capitalized : {:?}", input, input.to_uppercase());
     }
}

#[allow(dead_code)]
pub fn exercise(){
    exercise::test();
}