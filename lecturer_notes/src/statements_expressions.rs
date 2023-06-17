#![allow(dead_code)]
/*
From The Docs:
Statements and expressions:
Rust is primarily an expression language. This means that most forms of value-producing
or effect-causing evaluation are directed by the uniform syntax category of expressions.
Each kind of expression can typically nest within each other kind of expression, and
rules for evaluation of expressions involve specifying both the value produced by the
expression and the order in which its sub-expressions are themselves evaluated.
In contrast, statements serve mostly to contain and explicitly sequence expression evaluation.

Let's look at some examples:

Expressions:

    - Arithmetic operations such as addition (+), subtraction (-), multiplication (*), division (/) are expressions.

    - Function calls are expressions.

    - Control flow constructs are also expressions. That means that funky syntax like this is totally valid:

*/
// Not only will loop evaluate to 1, but this function will also return 1 any time it is called! 🤯 🤯 🤯
fn what_trickery_is_this() {

    let x= loop{
        break 1;
    };
    println!("x: {}", x);
}

/*

Statements:

    - Statements are instructions to perform some action and do not return a value.

    - Declaration statements like 'let' are used to bind a value to a variable.

Control flow expressions:

    - The 'match' construct works like a switch-case in other languages, and it's an expression in Rust.

    - Loop constructs, including 'for', 'while', and 'loop', are also expressions.

    - The value of a 'loop' expression is defined by a 'break' statement with a value.
*/


pub fn main() {
    // An arithmetic expression:
    let sum = 5 + 10; // The operation "5 + 10" is an expression that evaluates to 15.
    println!("The sum is: {}", sum);

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    // A function call is an expression:
    let result = add(5, 10); // The function call "add(5, 10)" is an expression that evaluates to 15.
    println!("The result is: {}", result);

    // 'if' control flow is an expression:
    let number = if sum == result { 5 } else { 10 };
    // The 'if' expression evaluates to 5 because sum and result are equal.
    println!("The number is: {}", number);

    // A statement:
    let _x = 5; // The 'let' declaration itself is a statement, it doesn't return a value.

    // A 'match' expression:
    let value = 1;
    let status = match value {
        1 => "one",
        2 => "two",
        _ => "other", // Default case
    };
    // The 'match' expression evaluates to "one" because 'value' is 1.
    println!("The status is: {}", status);

    // A 'loop' expression:
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count; // This 'break' statement returns a value, making the 'loop' an expression.
        }
    };
    // The 'loop' expression evaluates to 10 because 'count' reaches 10.
    println!("The result is: {}", result);
    what_trickery_is_this();
}
