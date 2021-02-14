const MAX_SPEED: i32 = 350;

fn clamp_speed(speed: i32) -> i32 {
    if speed <= MAX_SPEED {
        speed
    } else {
        MAX_SPEED
    }
}

pub fn practice(){
    let speed = clamp_speed(400);
    println!("speed : {:?}", speed);
}