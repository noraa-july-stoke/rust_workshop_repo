/*
Primitive Data Types Activities:

Objective: Develop a deeper understanding of primitive types in Rust, especially distinguishing between
`&str` and `String`, and mastering the use of `.to_owned()`.

Activity 1: Numeric Types

Goal: Gain hands-on experience with integer and floating point types in Rust.

1. Declare a variable named `x` of type `i32` and assign it the value 5. Print the value of `x` using the `println!` macro.
   Hint: The syntax for declaring a variable in Rust is `let` followed by the variable name, type, equals sign, and the value.
2. Now, declare a variable named `y` of type `f64` and assign it the value 5.0. Print the value of `y` using the `println!` macro.


Activity 2: String Types and Mutability

Goal: Understand the distinction between `String` and `&str` in Rust, especially relating to mutability.

    1. Declare a variable named `string1` as a `String` containing the text "Hello, world!". Print the value of `string1`.
    2. Update `string1` to append the phrase " It's a beautiful day!". This exercise aims to show that `String` type variables can be modified.
    3. Declare a variable named `string2` as a `&str` containing the text "Hello, world!". Print the value of `string2`.
    4. Try updating `string2` in the same way you updated `string1`. Observe the result and understand why it happens.
    5. Now, declare a variable named `string3` and assign it a reference to the `String` `string1`. Print the value of `string3`.

Activity 3: Creating and Manipulating Strings

Goal: Understand different ways to create `String` variables, and comprehend the usage and effect of `.to_owned()`.

    1. Use the `String::from()` method to create a new `String` named `new_string1` from the string literal "Hello, Rust!". Print the value of `new_string1`.
    2. Use the `.to_string()` method to create a new `String` named `new_string2` from the string literal "Hello, Rust!". Print the value of `new_string2`.
    3. Use the `.to_owned()` method to create a new `String` named `new_string3` from the `String` `string1`. Print the value of `new_string3`.
    4. Update `new_string3` by appending " Hello again, Rust!", and then print the updated `new_string3`.
    5. Print `string1` again to illustrate that `new_string3` is a separate copy, and that updating `new_string3` did not affect `string1`.
    6. Convert the integer `42` to a `String` using the `.to_string()` method. Assign the result to a variable named `new_string4`. Print the value of `new_string4`.
    7. Declare a string slice (`&str`) named `new_string5` and assign it the value "Hello, Rust!". Print the value of `new_string5`. Can you convert this `&str` to a `String` using `.to_owned()`?

Instructions:
1. Follow the goals and hints provided for each activity.
2. Code along, and take a moment to understand the output of each line of code.
3. Feel free to experiment and make mistakes, that's part of the learning process!
*/

pub fn main() {
    // Activity 1: Numeric Types
    let x: i32 = 5;
    println!("x = {}", x);

    let y: f64 = 5.0;
    println!("y = {}", y);

    // Activity 2: String Types
    let string1: String = String::from("Hello, world!");
    println!("string1 = {}", string1);

    // Let's append some text to string1 to illustrate its mutability
    let mut string1 = string1; // Remove `mut` and see what happens
    string1.push_str(" It's a beautiful day!");
    println!("string1 (updated) = {}", string1);

    let string2: &str = "Hello, world!";
    println!("string2 = {}", string2);

    // Since `string2` is a `&str`, we can't append to it like we did with `string1`.
    // Try uncommenting the next line and see what error you get.
    // string2.push_str(" It's a beautiful day!");


    // Activity 3: Creating Strings
    let new_string1: String = String::from("Hello, Rust!");
    println!("new_string1 = {}", new_string1);

    let new_string2: String = "Hello, Rust!".to_string();
    println!("new_string2 = {}", new_string2);

    // Here, we're creating an owned `String` from `string1` using `to_owned()`. This operation clones `string1` into `new_string3`.
    // The original `string1` is still valid and accessible. `new_string3` is a separate copy.
    let mut new_string3: String = string1.to_owned();
    println!("new_string3 = {}", new_string3);

    // Let's append some text to `new_string3` to illustrate it's a separate String from `string1`
    new_string3.push_str(" Hello again, Rust!");
    println!("new_string3 (updated) = {}", new_string3);
    println!("string1 (unchanged) = {}", string1); // Notice that `string1` remains unchanged

    let new_string4: String = 42.to_string();
    println!("new_string4 = {}", new_string4);

    let new_string5: &str = "Hello, Rust!";
    println!("new_string5 = {}", new_string5);

}
