#[derive(Debug)]
struct User {
    user_id: i64,
    name: String
}

fn find_user(name: &str) -> Option<i64> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(3),
        "katie" => Some(9),
        _ => None
    }
}

pub fn practice() {
    let user_name = "sam";
    let user = find_user(user_name).map(|user_id| User {
        user_id, name: user_name.to_owned()
    });

    match user {
        Some(user) => println!("{:?}", user),
        None => println!("can't not find user.")
    }
}