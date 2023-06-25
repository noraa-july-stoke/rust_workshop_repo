# Rust Cheat Sheet

## Basics

### Macros

- `println!()`: Prints a formatted string to the console.
- `format!()`: Creates a formatted string.
- `vec![]`: Creates a vector (growable array) from a list of items.

### Functions
```rust
fn functionName(parameter1: Type, parameter2: Type) -> ReturnType {
// Function body
// Return statement: return value;
}
```


### Access Modifiers

- `pub`: Marks an item as public, allowing it to be accessed from outside the current module.

### Types

- `i32`: 32-bit signed integer.
- `f64`: 64-bit floating-point number.
- `String`: Growable string.
- `Vec<T>`: Vector (growable array) of type `T`.

### Variables

- Declaration: `let variableName = value;`
- Mutable variables: `let mut variableName = value;`

### Constants

- Declaration: `const CONSTANT_NAME: Type = value;`

### Code Comments

- Single-line comment: `// Comment text`
- Multi-line comment: `/* Comment text */`

## Example Usage

```rust
fn main() {
    // Macros
    println!("Hello, world!");

    let s = format!("Hello, world!");
    println!("{}", s);

    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v);
    v.push(4);
    println!("{:?}", v);

    // Functions
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let result = add(3, 4);
    println!("Result: {}", result);

    // Variables
    let x = 5;
    let mut y = 10;
    y = y + x;
    println!("y: {}", y);

    // Constants
    const MAX_VALUE: i32 = 100;
    println!("Max Value: {}", MAX_VALUE);

    // Code comments
    // This is a comment

    /*
    This is a multi-line comment.
    */

    /*
    You can also disable warnings for the entire file by adding the following line to the top of the file:
    #![allow(dead_code)]
    */
}
