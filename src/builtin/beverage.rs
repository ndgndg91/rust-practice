// Debug to use "{:?}" token
// Clone, Copy not to use onwership, borrow, so always copy variable
// Clone, Copy is so expensive. so carafully use Clone, Copy Struct that have 3 ~ 4 entries
#[derive(Debug, Clone, Copy)]
enum Flavor {
    ORANGE,
    REMON,
    APPLE,
}
#[derive(Debug, Clone, Copy)]
struct Beverage {
    flavor: Flavor,
    pub fluid_oz: f64,
}

impl Beverage {
    /// create APPLE Flavor Beverage factory pattern
    fn apple() -> Self {
        Self {
            flavor: Flavor::APPLE,
            fluid_oz: 30.5,
        }
    }
    fn remon() -> Self {
        Self {
            flavor: Flavor::REMON,
            fluid_oz: 35.5,
        }
    }
    fn print_self(&self) {
        let description = match &self.flavor {
            Flavor::ORANGE => "ORANGE",
            Flavor::REMON => "REMON",
            Flavor::APPLE => "APPLE",
        };
        println!("{:?}, {:?}", description, self.fluid_oz)
    }
}

pub fn practice() {
    let apple_bever = Beverage::apple();
    apple_bever.print_self();
    print_flavor(&apple_bever.flavor);
    let remon_bever = Beverage::remon();
    remon_bever.print_self();
    let apple_beverage = Beverage {
        flavor: Flavor::APPLE,
        fluid_oz: 39.1,
    };
    println!("{:?}", apple_beverage);
    // apple_beverage.print_self();
    let remon_beverage = Beverage {
        flavor: Flavor::REMON,
        fluid_oz: 31.9,
    };
    println!("{:?}", remon_beverage);
    // remon_beverage.print_self();
    let orange_beverage = Beverage {
        flavor: Flavor::ORANGE,
        fluid_oz: 32.7,
    };
    println!("{:?}", orange_beverage);
    // orange_beverage.print_self();

    just_vector();
}

fn print_flavor(f: &Flavor) -> &'static str {
    return match f {
        Flavor::APPLE => "apple",
        Flavor::REMON => "remon",
        Flavor::ORANGE => "orange",
    };
}

fn just_vector() {
    let beverages = vec![Beverage::apple(), Beverage::remon()];
    for b in &beverages {
        b.print_self();
    }

    println!("beverage length : {:?}", beverages.len());

    // after for loop beverages not available by ownership
    for b in beverages {
        println!("fluid oz : {:?}", b.fluid_oz);
    }
}