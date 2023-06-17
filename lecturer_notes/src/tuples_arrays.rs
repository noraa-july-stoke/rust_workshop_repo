#![allow(dead_code)]
/*
Tuples in Rust are similar to arrays, but they can contain elements of different types.
Unlike Vectors, they are of fixed size, and you cannot add or remove elements after declaration.
One thing to note is that tuples do not have associated methods like Vectors.

Accessing elements in a tuple:

    Elements in a tuple can be accessed using a dot followed by the index of the element.
    Remember that Rust uses 0-based indexing, so the first element is at index 0.

Destructuring tuples:

    Rust supports destructuring tuples, which means that you can assign the elements of a tuple
    to separate variables in a single operation. This can be very useful in certain scenarios.

Function that accepts a tuple:

    You can also create functions that accept tuples as arguments or return tuples.

*/

pub fn tuples() {
    // First, let's declare a tuple:
    let my_tuple = (1, "Hello", 3.14);

    // Accessing elements in the tuple using index:
    println!("{}", my_tuple.0); // Prints: 1
    println!("{}", my_tuple.1); // Prints: Hello
    println!("{}", my_tuple.2); // Prints: 3.14

    // Destructuring the tuple into separate variables:
    let (a, b, c) = my_tuple;
    println!("{}", a); // Prints: 1
    println!("{}", b); // Prints: Hello
    println!("{}", c); // Prints: 3.14

    // Function that accepts a tuple and returns a tuple.
    fn swap((x, y): (i32, i32)) -> (i32, i32) {
        // This function takes a tuple of two integers and returns a new tuple with the integers swapped.
        (y, x)
    }

    // Creating and using a function that accepts a tuple:
    let swapped = swap((1, 2));
    println!("{:?}", swapped); // Prints: (2, 1)
}

/*

Arrays:

Arrays in Rust are homogeneous data structures, which means they can store multiple
values of the same data type. They have a fixed size that is known at compile time.

Some differences with arrays in Python and JavaScript:

- Fixed size: Unlike Python and JavaScript, the size of an array in Rust is fixed at the time of declaration.

- Type-safety: Rust arrays are type-safe, meaning that they can only contain elements of the same type.
  This contrasts with Python and JavaScript, where arrays (or lists, in Python terminology) can contain elements of different types.

- Indexing: Similar to Python and JavaScript, Rust also uses zero-based indexing.
  However, attempting to access an element out of bounds in Rust will cause a compile error, making Rust's arrays safer.

Here are some ways to work with arrays in Rust:

pub fn len(&self) -> usize:

    Returns the number of elements in the array.

pub fn is_empty(&self) -> bool:

    Returns true if the array has a length of 0.

pub fn contains(&self, x: &T) -> bool:

    Returns true if the array contains an element with the given value.

pub fn first(&self) -> Option<&T>:

    Returns a reference to the first element of the array, or None if it is empty.

*/

pub fn arrays() {
    // ARRAYS

    // Declaring an array:
    let arr = [1, 2, 3, 4, 5];

    // Getting the length of the array:
    let arr_length = arr.len();
    println!("The array has {} elements.", arr_length); // Prints: The array has 5 elements.

    // Checking if the array is empty:
    let is_empty = arr.is_empty();
    println!("Is the array empty? {}", is_empty); // Prints: Is the array empty? false

    // Checking if the array contains a certain value:
    let contains_three = arr.contains(&3);
    println!("Does the array contain the number 3? {}", contains_three); // Prints: Does the array contain the number 3? true

    // Getting the first element of the array:
    let first_element = arr.first();
    match first_element {
        Some(val) => println!("The first element is {}.", val), // Prints: The first element is 1.
        None => println!("The array is empty."),
    }

    // Just like with the vec macro, we can use shorthand to fill an array with repeating values:

    let matx= [[0; 4]; 5];
    println!("{:?}", matx); // Prints: [0, 0, 0, 0]
}

pub fn main() {
    arrays();
    tuples();
}
