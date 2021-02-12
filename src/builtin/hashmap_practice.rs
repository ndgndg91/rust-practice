use std::collections::HashMap;

#[derive(Debug)]
struct Container {
    description: String
}

fn print_values(map: &HashMap<i32, Container>) {
    for value in map.values() {
        println!("value : {:?}", value);
    }
}

fn print_keys(map: &HashMap<i32, Container>) {
    for key in map.keys() {
        println!("key : {:?}", key);
    }
}

fn print_keys_values(map: &HashMap<i32, Container>) {
    for (key, value) in map.iter() {
        println!("key : {:?}, value : {:?}", key, value);
    }
}

fn print_len(map : &HashMap<i32, Container>) {
    println!("containers len : {:?}", map.len());
}

pub fn practice(){
    let mut containers = HashMap::new();
    containers.insert(1, Container{description:String::from("first container")});
    containers.insert(2, Container{description:String::from("second container")});
    containers.insert(3, Container{description:String::from("third container")});
    
    print_keys_values(&containers);
    print_len(&containers);
    containers.remove(&3);

    print_keys_values(&containers);
    print_len(&containers);

    containers.insert(2, Container{description:String::from("overwrite test")});

    print_keys_values(&containers);
    print_len(&containers);
    
    print_keys(&containers);
    print_keys(&containers);
    print_values(&containers);
    print_values(&containers);
}