#![allow(dead_code)]

/*
Macros in Rust:

Rust provides a powerful macro system that allows metaprogramming, a way of writing code that writes or manipulates other code.
Macros look like functions, except that they operate on the code you write at compile time.
*/

/*
Built-in Macros:

Rust has several built-in macros that are frequently used:

1. `println!` is a macro that prints output to the console. It's more than just a function because it can take a variable
   number of arguments and can use format specifiers.

2. `vec!` is another commonly used macro, it allows us to initialize a vector with specified values.

3. `#[derive()]` is a macro used for automatically implementing traits such as Clone, Debug, PartialEq, or others for a
   given struct or enum.

4. `#![allow(dead_code)]` is a macro for compiler directives or 'attributes'. It tells the Rust compiler to stop warning
   about code which is never used.
*/

/*
Creating Declarative Macros:

These are the simplest form of macros and are declared using `macro_rules!`.
Declarative macros perform textual substitution and are often used for code generation.
*/



/*
Creating Procedural Macros:

Procedural macros are more complex and powerful than declarative macros. They operate on the Rust abstract syntax tree at compile time,
which means they can transform the code in more sophisticated ways, as seen in custom derive attributes.
*/

// Procedural macros are typically defined in their own crate. But for demonstration, here's an example of how you might
// define a custom derive procedural macro.

// #[derive(CustomDebug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

pub fn main() {

}
