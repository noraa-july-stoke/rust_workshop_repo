oub fn main() {
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
