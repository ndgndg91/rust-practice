#[allow(dead_code)]
enum Role {
    MANAGER,
    ADMIN,
    USER,
    GUEST,
}

struct Employee {
    name: String,
    role: Role
}

struct Door {

}

impl Door {
    fn access(&self, role: &Role) -> bool {
        // admin only allowed.
        return match role {
            Role::ADMIN => true,
            _ => false,
        };
    }
}

pub fn employee(){
    let door = Door{};
    let name = String::from("Name Dong Gil");
    let employee = Employee { name: name, role: Role::MANAGER };
    println!("{:?} can access : {:?}", &employee.name, door.access(&employee.role));
    println!("{:?}", &employee.name);
    println!("{:?}", &employee.name);
}