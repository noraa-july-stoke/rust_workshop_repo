#![allow(dead_code)]
/*
Data Types:

In Rust, variables have specific data types that determine the kind of values they can hold. This
section provides an overview of different data types used in Rust.

Basic Numeric Types:

    - i32: It is a 32-bit signed integer, capable of representing both positive and
      negative numbers. For example, `let x: i32 = 5;`.
    - f64: It is a 64-bit floating-point number, used to represent decimal values with
      high precision. For example, `let x: f64 = 5.0;`.
    - There are more numeric data types than this such as u32, u128, but these are the
      most commonly used ones.
*/

fn basic_numeric() {
    let x: i32 = 5;
    println!("x = {}", x);

    let x: f64 = 5.0;
    println!("x = {}", x);
}

/*
Basic String Types:

    - String: It is a growable string type in Rust. The size of a String can change
      at runtime, and it is guaranteed to be a valid UTF-8 sequence. Strings are
      stored as a vector of bytes (`Vec<u8>`) internally. For example,
      `let x: String = "Hello, world!".to_string();`.

    - &str: It is a string slice or a reference to a string. String slices have a
      known size at compile time and are commonly used to work with string literals.
      For example, `let x: &str = "Hello, world!";`.

    - &String: It is a reference to a String. Similar to `&str`, it allows referencing
      an existing `String` without taking ownership. It is useful when you need to pass
      a string reference to a function without transferring ownership. For example,
      `let x: &String = &"Hello, world!".to_string();`.

    - There are multiple ways to create &strs and Strings in Rust:

        1. String::from():
            - This method creates a new `String` by taking ownership of a value. It is a common
            way to create a `String` from other types.
            - Example: `let s: String = String::from("Hello, world!");`

        2. "string".to_string():
            - This method creates a new `String` by converting a string literal (`&str`) into a `String`.
            - Example: `let s: String = "Hello, world!".to_string();`

        3. .to_owned():
            - This method creates a new `String` by making a copy of an existing `String` or `&str`.
            - Example: `let s: String = "Hello, world!".to_owned();`

        4. .to_string():
            - This method creates a new `String` by converting the value to a `String` using the `ToString` trait.
            - Example: `let n: i32 = 42; let s: String = n.to_string();`

        5. &str:
            - String literals (enclosed in double quotes) are of type `&str`, which is a string slice.
            - Example: `let s: &str = "Hello, world!";`

    It's important to note that the resultant type depends on the method used. Methods like `String::from()`,
    `.to_string()`, and `.to_owned()` produce a `String` type, while string literals (`&str`) are naturally string slices.
*/

fn basic_string() {
    // The following are all valid ways to create a String.
    let x: String = "Hello, world!".to_string();
    println!("x = {}", x);

    let x: String = "Hello, world!".to_owned();
    println!("x = {}", x);

    let x: String = String::from("Hello, world!");
    println!("x = {}", x);

    // &str is a string slice.
    // This means that the size of the string is known at compile time.
    let x: &str = "Hello, world!";
    println!("x = {}", x);

    // &String is a reference to a String.
    let x: &String = &"Hello, world!".to_string();
    println!("x = {}", x);

    // Reversing a string
    let mut x: String = "Hello, world!".to_string();
    x = x.chars().rev().collect::<String>();
    println!("x = {}", x);
}

/*
Basic Compound Types:

    -   Struct: It is a user-defined type that can hold multiple named values. Structs allow
        you to define custom data structures with their own fields and behavior. For example:

    struct Person {
        name: String,
        age: i32,
        favorite_color: fn() -> String,
    };

    -   Structs share a lot of similarities with object and classes in other
        languages, but they are a bit different in Rust. Structs are used to
        create custom data types that can be used in your program. They are
        similar to tuples, but they have names associated with their data
        fields. One of the biggest differences between structs and their
        analogues in other languages is that structs in Rust do not have
        methods. Structs are exactly what you make them, and nothing else.
        There are no built-in methods, no magic methods, and no constructors.
        You must define all of these yourself. This provides a lot of
        flexibility, but does mean you may have to put in some work to
        implement the functionality you want. The impl keyword is used to
        implement functionality on a struct such as methods.
        Instead, you can define functions that take structs as
        arguments.
*/

struct Person {
    name: String,
    age: i32,
    favorite_color: fn() -> String,
}

/*
    -   Enum: It is a user-defined type that can contain multiple variants. Enums are useful when
        you want to represent a value that can have different states or options. Each variant
        can have its own associated data.
*/
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Associating data to each variant of an enum.
enum PersonEnum {
    Name(String),
    Age(i32),
    FavoriteColor(fn() -> String),
}

/*
    -   Tuple: It is a type that can hold multiple values of different types. Tuples are ordered
        collections where each element can have a different data type. If you need to store more
        than 2-3 values, it is recommended to use a struct instead.

*/

fn basic_tuple() {
    let x: (i32, f64, String) = (5, 5.0, "Hello, world!".to_string());
    println!("x = {:?}", x);
}

/*
The option type:
    - represents an optional value
    - every option is either some and contains a value, or none, and does not
    - option types are very common in rust code, as they have a wide range of uses
    - a common place you'll see them is in the return type of functions that may fail
    - it is an enumeration with two variants. Some and None.


    Useful when needing to work with optional data.
*/
enum Option<T> {
    Some(T),
    None,
}

/*
Note on References:
    -   The '&' symbol in front of a type indicates that it is a reference. References are
        pointers to values in memory and allow borrowing data without taking ownership.
        In Rust, references are distinct from raw pointers or owning pointers. We will
        cover references in more detail later.
*/

pub fn main() {
    basic_numeric();
    basic_string();
    basic_tuple();
}
