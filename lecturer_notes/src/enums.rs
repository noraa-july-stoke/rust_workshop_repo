#![allow(dead_code)]
use std::fmt;

/*
Enums in Rust:

Rust has a powerful feature "enum" that represents a type which could be one of several possible variants.
Each variant in an enum can optionally have data associated with it.
*/

/*
Simple enums:

    These are the simplest form of enums, which can be any variant of the enumeration.
*/

pub enum SimpleEnum {
    FirstVariant,
    SecondVariant,
    ThirdVariant,
}

// Try to print the enum before adding this to see what happens.
impl fmt::Display for SimpleEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SimpleEnum::FirstVariant => "First Variant",
                SimpleEnum::SecondVariant => "Second Variant",
                SimpleEnum::ThirdVariant => "Third Variant",
            }
        )
    }
}

fn show_enum() {
    println!("Simple enum: {}", SimpleEnum::FirstVariant);
}

/*
Enums with associated data:

    This kind of enum allows us to store data inside each variant.
    This is useful for wrapping multiple types of data into a single type.
*/

pub enum ComplexEnum {
    Int(i32),
    Float(f64),
    Text(String),
}

/*
Enums with methods:

    Enums can also have methods associated with them.
*/

pub enum State {
    Inactive,
    Active,
}

impl State {
    pub fn is_active(&self) -> bool {
        matches!(self, State::Active)
    }
}

pub fn main() {
    // Using simple enum
    let a = SimpleEnum::FirstVariant;

    match a {
        SimpleEnum::FirstVariant => println!("It's the first variant."),
        SimpleEnum::SecondVariant => println!("It's the second variant."),
        SimpleEnum::ThirdVariant => println!("It's the third variant."),
    }

    // Using enum with associated data
    let b = ComplexEnum::Text(String::from("Hello world!"));

    match b {
        ComplexEnum::Int(i) => println!("Integer: {}", i),
        ComplexEnum::Float(f) => println!("Float: {}", f),
        ComplexEnum::Text(t) => println!("Text: {}", t),
    }

    // Using enum with methods
    let state = State::Active;
    println!("Is state active? {}", state.is_active());

    show_enum();
}
