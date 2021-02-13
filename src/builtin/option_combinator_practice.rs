#[derive(Debug)]
enum Role {
    Manager,
    Admin,
    Normal,
    Observer
}

fn find_role(role: &str) -> Option<Role> {
    match role.to_lowercase().as_str() {
        "manager" => Some(Role::Manager),
        "admin" => Some(Role::Admin),
        "normal" => Some(Role::Normal),
        "observer" => Some(Role::Observer),
        _ => None
    }
}


pub fn practice() {
    let is_none = find_role("admin").is_none();
    dbg!(is_none);
    let is_some = find_role("admin").is_some();
    dbg!(is_some);
    let normal = find_role("none").or_else(|| Some(Role::Normal));
    dbg!(normal);

    let admin = find_role("none").unwrap_or_else(|| Role::Admin);
    dbg!(admin);
}