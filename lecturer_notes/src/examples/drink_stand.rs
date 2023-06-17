#![allow(dead_code)]
// fmt and io are used for printing to the console
use std::{fmt, io};
// FromStr is used for converting a string to a different type
use std::str::FromStr;

// Declare the DrinkSize enum
#[derive(Debug)]
enum DrinkSize {
    Small,
    Medium,
    Large,
}

// Implement the `FromStr` trait for `DrinkSize`, allowing a string to be converted to `DrinkSize`.
// impl FromStr for DrinkSize  means that we can use the `parse()` method on a string to convert it to a `DrinkSize`.
// The `parse()` method is used to convert a string to a different type.

impl FromStr for DrinkSize {
    type Err = ();
    fn from_str(input: &str) -> Result<DrinkSize, Self::Err> {
        match input {
            "Small" => Ok(DrinkSize::Small),
            "Medium" => Ok(DrinkSize::Medium),
            "Large" => Ok(DrinkSize::Large),
            _ => Err(()),
        }
    }
}

// Declare the Flavor enum
#[derive(Debug)]
enum Flavor {
    Lemon,
    Orange,
    Grape,
}

// Declare the Drink struct
#[derive(Debug)]
struct Drink {
    name: String,
    size: DrinkSize,
    flavor: Flavor,
    price: f64,
}

impl Drink {
    fn new(name: String, size: DrinkSize, flavor: Flavor, price: f64) -> Drink {
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
            "Drink: {}, Size: {:?}, Flavor: {:?}, Price: {:.2}",
            self.name, self.size, self.flavor, self.price
        )
    }
}

pub fn main() {
    let mut drinks = Vec::new();

    drinks.push(Drink::new(
        "Lemonade".into(),
        DrinkSize::Small,
        Flavor::Lemon,
        1.50,
    ));
    drinks.push(Drink::new(
        "Orange Juice".into(),
        DrinkSize::Large,
        Flavor::Orange,
        2.00,
    ));
    drinks.push(Drink::new(
        "Grape Soda".into(),
        DrinkSize::Medium,
        Flavor::Grape,
        1.75,
    ));

    loop {
        println!("Welcome to the Drink Stand!");
        println!("Here are our drinks:");

        for (i, drink) in drinks.iter().enumerate() {
            println!("{}. {}", i + 1, drink.name);
        }

        let drink_choice: usize;
        loop {
            let mut drink_choice_input = String::new();
            println!("Enter the number of the drink you want or 'q' to quit:");

            io::stdin()
                .read_line(&mut drink_choice_input)
                .expect("Failed to read line");

            if drink_choice_input.trim() == "q" {
                println!("Thank you for visiting our Drink Stand. Have a nice day!");
                return; // terminate the program
            }

            match drink_choice_input.trim().parse::<usize>() {
                Ok(num) => {
                    if num == 0 || num > drinks.len() {
                        println!("Invalid choice. Please enter a valid number.");
                        continue;
                    } else {
                        drink_choice = num;
                        break;
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            }
        }

        // Ask for the size
        let size_choice: DrinkSize;
        loop {
            let mut size_choice_input = String::new();
            println!("Enter the size of the drink you want (Small, Medium, Large):");

            io::stdin()
                .read_line(&mut size_choice_input)
                .expect("Failed to read line");

            match size_choice_input.trim() {
                "Small" => {
                    size_choice = DrinkSize::Small;
                    break;
                }
                "Medium" => {
                    size_choice = DrinkSize::Medium;
                    break;
                }
                "Large" => {
                    size_choice = DrinkSize::Large;
                    break;
                }
                _ => {
                    println!("Invalid size. Please enter a valid size.");
                    continue;
                }
            };
        }

        let chosen_drink = &mut drinks[drink_choice - 1];
        // Update the size of the drink
        chosen_drink.size = size_choice;

        chosen_drink.print();

        println!(
            "You chose a {:?} {}. That'll be ${:.2}. Please enter your payment:",
            chosen_drink.size, chosen_drink.name, chosen_drink.price
        );

        let mut payment = String::new();
        io::stdin()
            .read_line(&mut payment)
            .expect("Failed to read line");

        let payment: f64 = match payment.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid payment. Please enter a valid number.");
                continue;
            }
        };

        if payment < chosen_drink.price {
            println!("Insufficient payment. Please pay full amount.");
            continue;
        }

        println!(
            "Payment successful. Here is your {}. Enjoy!",
            chosen_drink.name
        );
    }
}
