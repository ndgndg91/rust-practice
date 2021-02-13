mod math;
mod builtin;
mod my;
mod user_input;

// This is Entry point of the applicaiton. 
// like public static void main(String[] args) {} in Java.
fn main() {
    // user_input::practice();
    // user_input::exercise();
    math::practice();
    builtin::practice();
    my::indirect_access();
    my::function();
}