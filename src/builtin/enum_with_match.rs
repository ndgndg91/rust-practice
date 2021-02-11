enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn practice() {
    // enum_with_match function has a go variable ownership.
    let go = Direction::UP;
    // which_way function take a go varialbe ownership. so, enum_with_match function loose a go variable ownership.
    which_way(go);
    // so, below code raise error.
    // which_way(go);
    which_way(Direction::DOWN);
    which_way(Direction::LEFT);
    which_way(Direction::RIGHT);
}

fn which_way(go: Direction) {
    match go {
        Direction::UP => println!("go up"),
        Direction::DOWN => println!("go down"),
        Direction::LEFT => println!("go left"),
        Direction::RIGHT => println!("go right"),
    }
}
