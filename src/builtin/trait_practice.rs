trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    side: i32
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side_a + self.side_b + self.side_c
    }
 }

 impl Default for Triangle {
     fn default() -> Self {
         Self {side_a: 3, side_b: 3, side_c: 3}
     }
 }

fn print_perimeter(shape: &impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("shape perimeter {:?}", perimeter);
}

pub fn practice(){
    let square = Square{ side: 5};
    let triangle = Triangle{
        side_a: 5,
        side_b: 3,
        side_c: 10
    };
    print_perimeter(&square);
    print_perimeter(&triangle);

    let default = Triangle::default();
    print_perimeter(&default);
}