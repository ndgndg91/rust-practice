mod math;
mod builtin;
mod my;

// This is Entry point of the applicaiton. 
// like public static void main(String[] args) {} in Java.
fn main() {
    math::practice();
    builtin::practice();
    my::indirect_access();
    my::function();
}