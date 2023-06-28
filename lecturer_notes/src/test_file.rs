#![allow(unused_variables)]
fn string_methods() {
    let s = String::from("Hello, ");

    // Concatenation and Appending
    let concatenated = s.clone() + "World!"; // Concatenates two strings
    let appended = format!("{}{}", s, "World!"); // Appends strings using format macro
    let mut mutable_s = String::from("Hello, ");
    mutable_s.push_str("World!"); // Appends a string slice to the string

    // Slicing and Indexing
    let sliced = &s[0..5]; // Slices a portion of the string
    let first_char = s.chars().next(); // Returns the first character as an Option<char>

    // Length and Capacity
    let length = s.len(); // Returns the length of the string in bytes
    let capacity = s.capacity(); // Returns the current capacity of the string in bytes

    // Iteration and Manipulation
    for c in s.chars() {
        // Iterates over the characters of the string
        println!("{}", c);
    }
    let replaced = s.replace("l", "L"); // Replaces all occurrences of a substring
    let trimmed = s.trim(); // Removes leading and trailing whitespaces
    let uppercase = s.to_uppercase(); // Converts the string to uppercase
    let lowercase = s.to_lowercase(); // Converts the string to lowercase

    // Searching and Checking
    let contains = s.contains("ell"); // Checks if the string contains a substring
    let starts_with = s.starts_with("Hel"); // Checks if the string starts with a substring
    let ends_with = s.ends_with("lo!"); // Checks if the string ends with a substring
    let is_empty = s.is_empty(); // Checks if the string is empty

    // Conversion
    let integer = s.parse::<i32>(); // Parses the string into an integer

    // Printing
    println!("{}", s); // Prints the string
}

fn vector_methods() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Accessing Elements
    let first_element = v[0]; // Accesses an element at a specific index
    let last_element = v.last(); // Returns an Option reference to the last element

    // Adding and Removing Elements
    v.push(6); // Appends an element to the end of the vector
    let popped_element = v.pop(); // Removes and returns the last element

    // Iteration and Manipulation
    for element in &v {
        // Iterates over the elements of the vector
        println!("{}", element);
    }
    v.sort(); // Sorts the elements in ascending order
    v.reverse(); // Reverses the order of elements

    // Slicing
    let slice = &v[1..3]; // Slices a portion of the vector

    // Length and Capacity
    let length = v.len(); // Returns the number of elements in the vector
    let capacity = v.capacity(); // Returns the current capacity of the vector

    // Searching and Checking
    let contains = v.contains(&3); // Checks if the vector contains a specific element
    let index = v.iter().position(|&x| x == 4); // Returns the index of the first occurrence of an element

    // Conversion
    let string = v.clone()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>(); // Converts the vector elements into strings

    // Printing
    println!("{:?}", v); // Prints the vector

}

pub fn main() {
    string_methods();
    vector_methods();
}
