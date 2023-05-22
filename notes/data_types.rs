// Data Types:


pub fn main() {

    // Basic numeric types

    // i32 is a 32-bit signed integer.
    let x: i32 = 5;
    println!("x = {}", x);

    // f64 is a 64-bit floating point number.
    let x: f64 = 5.0;
    println!("x = {}", x);


    // Basic string types:
    // String is a growable string.
    // This means that the size of the string can change at runtime.
    // A String is stored as a vector of bytes (Vec<u8>), but guaranteed
    // to always be a valid UTF-8 sequence.
    let x: String = "Hello, world!".to_string();

    // &str is a string slice.
    // This means that the size of the string is known at compile time.
    let x: &str = "Hello, world!";

    // &String is a reference to a String.
    // This means that the size of the string can change at runtime.
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
