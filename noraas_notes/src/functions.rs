#![allow(dead_code)]
/*
Functions in Rust:

Rust is an imperative language, which means it has a clear concept of a function: a procedure that can be executed on command.
Functions are declared with the `fn` keyword and can accept arguments and return a value.
*/

// A simple function that prints "Hello, world!"
fn say_hello() {
    println!("Hello, world!");
}

// A function that accepts two arguments and returns their sum.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*
Closures in Rust (Anonymous Functions):

Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
They are a core feature of many functional programming languages.
Closures can capture values from their surrounding scope, creating a sort of 'function plus environment'.
*/

fn closures() {
    // A simple closure that prints "Hello, world!"
    let say_hello = || {
        println!("Hello, world!");
    };

    // A closure that accepts two arguments and returns their sum.
    let add = |a: i32, b: i32| a + b;

    // A closure that captures a variable from its environment
    let x = 5;
    let add_x = |a: i32| a + x;
}




pub fn main() {
    // Using regular functions
    say_hello();
    let sum = add(5, 6);
    println!("The sum is {}", sum);

    // Using closures
    say_hello();
    let sum = add(5, 6);
    println!("The sum is {}", sum);
    let sum = add_x(5);
    println!("The sum is {}", sum);
}
