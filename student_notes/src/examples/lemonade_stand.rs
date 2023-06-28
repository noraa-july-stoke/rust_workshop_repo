#![allow(dead_code)]
use std::{fmt, io, str};

// An enum is a way of making custom types
#[derive(Debug)]
enum DrinkSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
enum Flavor {
    Lemonade,
    Orange,
    Grape,
}

struct Drink {
    name: String,
    size: DrinkSize,
    flavor: Flavor,
    price: f64,
}

impl Drink {
    fn new(name: String, size: DrinkSize, flavor: Flavor, price: f64) -> Self {
        Drink {
            name,
            size,
            flavor,
            price,
        }
    }

    fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Drink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Drink: {}, Size: {:?}, Flavor: {:?}, Price:: {:.2}",
            self.name, self.size, self.flavor, self.price
        )
    }
}
