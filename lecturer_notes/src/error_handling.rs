#![allow(dead_code)]
/*
Error Handling in Rust:

Rust uses two main types for error handling: Result and Option, and has a number of methods and macros for handling errors effectively.

Unwrap:

The unwrap method is available for both Result and Option types. It is used to retrieve the inner value of these types. However, its behavior differs depending on whether it is called on a Result or an Option.

1. On a Result:
    If called on an Ok variant, unwrap will return the value inside.
    If called on an Err variant, unwrap will panic, terminating the program.

2. On an Option:
    If called on a Some variant, unwrap will return the value inside.
    If called on a None variant, unwrap will panic, terminating the program.

Because of the potential for panics, unwrap is generally best used when you have a strong reason to believe
that the Result or Option will be Ok/Some, or during prototyping where you intend to handle the errors later.
*/

enum Option<T> {
    None,
    Some(T),
}

impl<T> Option <T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(_) => panic!("called `Result::unwrap()` on an `Err` value"),
        }
    }
}

pub fn main() {
    let some_number: Option<i32> = Option::Some(10);
    let unwrapped_number = some_number.unwrap();
    println!("Unwrapped number: {}", unwrapped_number);

    let successful_result: Result<&str, &str> = Result::Ok("success");
    let unwrapped_result = successful_result.unwrap();
    println!("Unwrapped result: {}", unwrapped_result);
}
