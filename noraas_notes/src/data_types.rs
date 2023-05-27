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

Creating Strings:

    In Rust, there are several ways to create strings. Depending on the method used, the
    resultant type can be either a `String` or a `&str` (string slice).
    Let's explore some of the common methods:

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

    When choosing the appropriate method, consider whether you need ownership of the string or a
    reference to an existing string. Additionally, remember that `String` types are growable and
    can be modified, while string slices (`&str`) have a fixed size and are often used for
    read-only access or passing references.

    By understanding these different ways to create strings and their resultant types,
    you can effectively work with strings in Rust.

None type:

    Rust has a none type. This is very similar to null and None in other languages. It is used
    to represent the absence of a value.



Basic Compound Types:

    -   Struct: It is a user-defined type that can hold multiple named values. Structs allow
        you to define custom data structures with their own fields and behavior. For example:

    struct Person {
        name: String,
        age: i32,
        favorite_color: fn() -> String,
    };

    -   Enum: It is a user-defined type that can contain multiple variants. Enums are useful when
        you want to represent a value that can have different states or options. Each variant
        can have its own associated data. For example:

        enum Person {
            Name(String),
            Age(i32),
            FavoriteColor(fn() -> String),
        }

    -   Tuple: It is a type that can hold multiple values of different types. Tuples are ordered
        collections where each element can have a different data type. If you need to store more
        than 2-3 values, it is recommended to use a struct instead. For example:

    let x: (i32, f64, String) = (5, 5.0, "Hello, world!".to_string());


Note on References:
    -   The '&' symbol in front of a type indicates that it is a reference. References are
        pointers to values in memory and allow borrowing data without taking ownership.
        In Rust, references are distinct from raw pointers or owning pointers. We will
        cover references in more detail later.

These are the basic data types in Rust. Understanding and using the appropriate data types is essential for effective programming in Rust.
*/

pub fn main() {
    // Basic numeric types

    // i32 is a 32-bit signed integer.
    let x: i32 = 5;
    println!("x = {}", x);

    // f64 is a 64-bit floating point number.
    let x: f64 = 5.0;
    println!("x = {}", x);

    // Basic string types:
    // Char: A single character. Unicode scalar value.
    // String: a growable string.
    // This means that the size of the string can change at runtime.
    // A String is stored as a vector of bytes (Vec<u8>), but guaranteed
    // to always be a valid UTF-8 sequence.
    let x: String = "Hello, world!".to_string();

    // &str is a string slice.
    // This means that the size of the string is known at compile time.
    let x: &str = "Hello, world!";

    // &String is a reference to a String.
    let x: &String = &"Hello, world!".to_string();

    /*
    Quick note: the '&' in front of a type means that the type is a reference.
    A reference is a pointer to a value in memory, but it is different than a
    raw pointer or an owning pointer. We will cover references in more detail later.
    References are denoted by the & symbol followed by the referenced type. For
    example, &i32 is an immutable reference to an i32, and &mut String is a mutable
    reference to a String.
    */

    // Basic compound types
    // Struct is a user-defined type that can contain multiple values.
    struct Person {
        name: String,
        age: i32,
        favorite_color: fn() -> String,
    };

    // Enum is a user-defined type that can contain multiple variants.
    enum Person {
        Name(String),
        Age(i32),
        FavoriteColor(fn() -> String),
    }

    // Tuple is a type that can contain multiple values. If you need to hold
    // more than 2-3 values, it is recommended you use a struct instead.
    let x: (i32, f64, String) = (5, 5.0, "Hello, world!".to_string());
}
