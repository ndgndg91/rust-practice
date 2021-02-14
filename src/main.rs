mod math;
mod builtin;
mod my;
mod user_input;


#[allow(dead_code)]
fn all_caps(word: &str) -> String {
    return word.to_uppercase();
}

// This is Entry point of the applicaiton. 
// like public static void main(String[] args) {} in Java.
fn main() {
    // user_input::practice();
    // user_input::exercise();
    math::practice();
    builtin::practice();
    my::indirect_access();
    my::function();
    self_lib::print_hi_in_lib();
}



#[cfg(test)]
mod testing {
    use crate::*;
    #[test]
    fn check_all_caps(){
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should be all uppercase."); 
    }
}