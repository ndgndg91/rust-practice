// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

#[allow(dead_code)]
pub fn function() {
    println!("called `my::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::private_function()`");
}

#[allow(dead_code)]
pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
