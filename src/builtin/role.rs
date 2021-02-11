#[allow(dead_code)]
enum Role {
    MANAGER,
    ADMIN,
    USER,
    GUEST,
}

fn just_expression() {
    let role = Role::GUEST;
    // admin only allowed.
    let can_access_file = match role {
        Role::ADMIN => true,
        _ => false,
    };

    println!("can access ? {:?}", can_access_file);
}