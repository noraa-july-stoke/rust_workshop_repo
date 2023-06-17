#![allow(dead_code)]

/*
Structs in Rust:

Rust has three types of structures "structs" that can be created for use in a program:
named-field structs, tuple structs, and unit structs. They differ in how their fields are accessed.
*/

/*
Named-field structs:

    This is the most common type of struct and consists of an optional visibility, a name,
    and named fields. Each field has a type, and its name is used to access the data it contains.
*/

pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    pub fn print(&self) {
        println!("The person's name is {} and age is {}.", self.name, self.age)
    }
}

/*
Tuple structs:

    These are structs that have fields with types but no names. These fields can be accessed by
    using dot notation along with the index of the field (like tuples).
*/

pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn print(&self) {
        println!("RGB values for black color are: {}, {}, {}", self.0, self.1, self.2)
    }
}

/*
Unit structs:

    These are structs without any fields. They are used primarily for generic programming.
    While at first glance they might seem useless (after all, they don't actually store any data),
    unit structs can actually be quite useful in certain scenarios.

    Marker or Phantom types: They can be used to create distinct types for type safety and clarity,
    even when they don't need to carry data. For example, you might have two unit structs, Inches and Centimeters,
    that you use as types for function parameters to make sure you don't mix them up.

    Traits and trait objects: If you have a trait that doesn't need any data, you can implement it for a unit
    struct and then use trait objects of that type.
*/

pub struct Unit;
pub struct Inches;
pub struct Centimeters;

impl Inches {
    pub fn print_length_in_inches(len: u32) {
        println!("The length is {} inches.", len);
    }
}

impl Centimeters {
    pub fn print_length_in_cm(len: u32) {
        println!("The length is {} centimeters.", len);
    }
}

pub fn main() {
    let person = Person::new("John Doe", 30);
    person.print();

    let black = Color::new(0, 0, 0);
    black.print();

    Inches::print_length_in_inches(10);
    Centimeters::print_length_in_cm(25);
}
