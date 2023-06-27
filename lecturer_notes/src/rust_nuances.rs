#![allow(dead_code)]
// ownership, borrowing, lifetimes, references, borrow checker, and pointers

/*

1. Value moves in rust.

Ownership and Move Semantics: Rust follows a strict ownership model,
where every value has a unique owner at any given time. When a value
is moved, the ownership is transferred from the source variable to the destination variable.

Single Assignment: Rust enforces the concept of single assignment,
meaning that a binding can only be assigned a value once. After a value
is moved, the source binding becomes invalid or uninitialized.

No Implicit Copy: Unlike some other languages, Rust does not automatically
create copies of values during assignments or function calls. Instead, it moves
the value by default. However, types that implement the Copy trait, such as integers
and certain structs, are copied instead of moved.

Borrowing and References: To access a value without transferring ownership, Rust
provides borrowing and references. Borrowing allows multiple immutable references (&)
or a single mutable reference (&mut) to a value, enabling temporary access
 without taking ownership.

Copy Types vs. Non-Copy Types: Copy types, as mentioned earlier, are types
that are implicitly copied when assigned or passed as arguments. Non-copy
types are moved by default. You can determine whether a type is Copy or not
by checking its trait implementation.
*/

// Borrowing example
fn borrow_func() {
    // Borrowing example
    fn print_length1(s: &String) {
        println!("Length: {}", s.len());
    }

    fn print_length2(s: String) {
        println!("Length: {}", s.len());
    }

    let my_string = String::from("Hello");
    print_length1(&my_string);
    println!("{}", my_string);

    print_length2(my_string);
}

// value move example
fn move_func() {
    let source = String::from("Hello");
    // Move the value from source to destination
    let destination = source;
    // println!("{}", source); // Error: value borrowed here after move
    println!("{}", destination); // Output: Hello
}

// weird loop example
// statement vs exp

fn loop_func() {
    let mut num: i32 = 5;

    // from "let weird" to the ending semicolon of loop is a "statement"
    let weird = loop {
        num += 1;
        if num == 5 {
            break num;
        }
    };

    println!("{weird}");

    // it's not a statement so no semicolon needed.
    loop {
        num += 1;
        if num == 10 {
            break;
        }
    }
}

pub fn main() {
    // borrow_func();
    // move_func();
}
