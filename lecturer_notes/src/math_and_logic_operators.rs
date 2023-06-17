#![allow(dead_code)] // Allow dead code throughout this module

/*
=================================================================================================================

Math and Logic Operators

In Rust, you have access to various math and logic operators that allow you to perform calculations and logical
operations. These operators enable you to manipulate values and make decisions based on conditions.
Here are some of the commonly used operators:

1. Arithmetic Operators:
   - Addition: +
   - Subtraction: -
   - Multiplication: *
   - Division: /
   - Remainder (Modulo): %
   - Increment: +=
   - Decrement: -=
   No ++ or -- operators in Rust :(


   These operators are used to perform basic arithmetic operations on numeric values. For example, you can
   add two numbers using the + operator, subtract them using the - operator, multiply them using the * operator,
   and divide them using the / operator. The remainder operator % calculates the remainder of the division. The
   increment operator += increases the value of a variable by a specific amount, and the decrement operator -=
   decreases the value of a variable by a specific amount.

2. Comparison Operators:
   - Equal to: ==
   - Not equal to: !=
   - Greater than: >
   - Less than: <
   - Greater than or equal to: >=
   - Less than or equal to: <=

   Comparison operators are used to compare values and evaluate conditions. They return a boolean value (true or
   false) based on the comparison result. For example, you can check if two values are equal using the == operator,
   or if one value is greater than another using the > operator. These operators are commonly used in conditional
   statements and loops to make decisions based on the comparison result.

3. Logical Operators:
   - Logical AND: &&
   - Logical OR: ||
   - Logical NOT: !
   - Bitwise OR: |   *Be aware that the syntax used for matching multiple expressions in a match statement is also
                     |, but it is not a bitwises operator.  It is a pattern matching operator specifically in the
                     context of a match statement.

   Logical operators are used to combine or negate boolean values. The logical AND operator && returns true if both
   operands are true. The logical OR operator || returns true if at least one of the operands is true. The logical
   NOT operator ! negates the value of a boolean expression. The bitwise OR operator | performs a bitwise OR operation
   on integer values. These operators are useful for constructing complex conditions and controlling the flow of
   execution based on multiple conditions.

4. Range Operators:
   - Inclusive Range: ..=
   - Exclusive Range: .. (unstable feature, will need to enable unstable features to use)

   Range operators are used to define a range of values.  The inclusive
   range operator ..= includes the end value as well. For example, 1..=5 represents the values from 1 to 5 (inclusive).
   The exclusive range operator .. excludes the end value. For example, 1..5 represents the values from 1 to 4 (inclusive).
   In rust, the exclusive range operator is an unstable feature, so you will need to enable unstable features to use it.
   Probably just don't use it, lol.

5. Math Libraries:
   - The standard library in Rust provides several math-related modules and functions that you can use for advanced
     mathematical operations. Some of the commonly used math modules include:
     - std::f32 and std::f64: These modules provide constants and functions for working with floating-point numbers.
     - std::num: This module provides numeric traits and functions for manipulating numbers.
     - std::cmp and std::cmp::Ordering: These modules provide comparison-related functions and enums for ordering and
       comparing values.

6. Random Number Generation:
Rust provides several ways to generate random numbers. Random numbers are often useful in various scenarios, such as games,
simulations, and cryptography. Here are a few options for generating random numbers in Rust:

    rand crate:
        - The `rand` crate is a widely-used crate for random number generation in Rust. It provides a high-quality source
          of random numbers based on the operating system's random number generator (RNG).

   To use the `rand` crate, you need to add it as a dependency in your Cargo.toml file:
    --------------cargo.toml----------------
    [dependencies]
    rand = "0.8"
    -----------------------------------------
   Then, you can use the rand::random function to generate random numbers. For example, the following
   code generates a random number between 1 and 10: `let random_number = rand::random::<u8>() % 10 + 1;`


Feel free to explore the Rust documentation and experiment with different operators, math libraries, and random
number generation techniques to enhance your programs. Have fun coding!
=================================================================================================================

*/

use rand::Rng; // Import the random number generator trait
const MAX_NUMBER: u8 = 10; // Define a constant for the maximum random number

pub fn main() {
    //  Math Operators
    println!("=== Math Operators ===");

    let num1: i32 = 10;
    let num2: i32 = 5;

    let sum = num1 + num2; // Addition
    println!("Sum: {}", sum);

    let difference = num1 - num2; // Subtraction
    println!("Difference: {}", difference);

    let product = num1 * num2; // Multiplication
    println!("Product: {}", product);

    let quotient = num1 / num2; // Division
    println!("Quotient: {}", quotient);

    let remainder = num1 % num2; // Modulo (Remainder)
    println!("Remainder: {}", remainder);

    let floating_point_num: f64 = 10.1;
    let integer_num: i32 = 5;

    fn add_float_and_int(floating_point_num: i32, integer_num: f64) {
        println!(
            "results of add_float_and_int: {}",
            (floating_point_num) + (integer_num as i32)
        )
    }

    add_float_and_int(floating_point_num as i32, integer_num as f64);

    println!(
        "Sum of float and int is {}:",
        floating_point_num as i32 + integer_num
    );

        let mut num3 = 3;
        num3 += 1; // Increment
        println!("Incremented num3: {}", num3);

        let mut num4 = 6;
        num4 -= 2; // Decrement
        println!("Decremented num4: {}", num4);

        println!("");

        // Comparison Operators
        println!("=== Comparison Operators ===");

        let a = 5;
        let b = 7;

        let equal = a == b; // Equal to
        println!("Equal: {}", equal);

        let not_equal = a != b; // Not equal to
        println!("Not Equal: {}", not_equal);

        let greater_than = a > b; // Greater than
        println!("Greater Than: {}", greater_than);

        let less_than = a < b; // Less than
        println!("Less Than: {}", less_than);

        let greater_than_or_equal = a >= b; // Greater than or equal to
        println!("Greater Than or Equal: {}", greater_than_or_equal);

        let less_than_or_equal = a <= b; // Less than or equal to
        println!("Less Than or Equal: {}", less_than_or_equal);

        println!("");

        // Logical Operators
        println!("=== Logical Operators ===");

        let x = true;
        let y = false;

        let logical_and = x && y; // Logical AND
        println!("Logical AND: {}", logical_and);

        let logical_or = x || y; // Logical OR
        println!("Logical OR: {}", logical_or);

        let logical_not = !x; // Logical NOT
        println!("Logical NOT: {}", logical_not);

    // /*
    //    BITWISE OR OPERATOR:

    //       In Rust, the bitwise OR operator | is primarily used for low-level operations, bit manipulation,
    //       and working with flags or bitmasks. It allows you to set specific bits or combine different bit
    //       patterns in integers.

    //       If you're looking for logical OR behavior and a boolean result, you should use the logical
    //       OR operator ||. It evaluates the truthiness of the operands and returns a boolean value
    //       based on their logical relationship.

    //       The bitwise OR operator | doesn't return a boolean value like the logical OR operator ||.
    //       it operates at the binary level, performing a bitwise OR operation on each corresponding bit
    //       of the integers involved. It combines the bits from the operands, evaluating each bit
    //       independently and producing a new integer as the result. Unlike the logical OR operator ||,
    //       which returns a boolean value (true or false) based on the truthiness of the operands,
    //       the bitwise OR operator | doesn't evaluate the truthiness or falseness of the operands.
    //       Instead, it focuses on manipulating the binary representation of the integers by performing
    //       the OR operation on each bit. Saying that the bitwise OR operator | focuses on manipulating
    //       the binary representation of integers by performing the OR operation on each bit, it means
    //       that the operator operates on the individual bits (0s and 1s) of the binary representation
    //       of the integers. In binary representation, each digit (bit) can have a value of either 0
    //       or 1. The bitwise OR operation is applied to the corresponding bits of two integers. It
    //       compares the bits at each position and produces a new integer where each bit is set to 1
    //       if either or both of the corresponding bits in the operands are 1.

    //       Here's an example to illustrate this:
    //       In this example, a and b are binary representations of integers. The bitwise OR operation
    //       a | b compares the corresponding bits of a and b. Starting from the rightmost bit, it
    //       performs the OR operation on each pair of bits:
    //       |
    //       |     1 0 1 0   (a)
    //       |   | 1 1 0 0   (b)
    //       |   -----------
    //       |     1 1 1 0   (result)
    //       |

    // */

    //     let bitwise_or = 0b1010 | 0b1100; // Bitwise OR
    //     println!("Bitwise OR: {:b}", bitwise_or);

    // /*
    //       Here is the bitwise diagram for the next example:
    //          a:  00001010
    //          b:  00001100
    //       ----------------
    //          OR: 00001110
    // */

        let a = 10; // Decimal representation of 10
        let b = 12; // Decimal representation of 12

        let bitwise_or = a | b; // Bitwise OR
        println!("Bitwise OR: {}", bitwise_or);

        println!("");

        // Inclusive Range
        println!("=== Inclusive Range ===");

        for num in 1..=5 {
            println!("Inclusive Range: {}", num);
        }

        println!("");

    //     // Random Number Generation
        println!("=== Random Number Generation ===");

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..=MAX_NUMBER); // Generate a random number between 1 and 10 (inclusive)
        println!("Random Number: {}", random_number);

        let result = random_number % 2; // Calculate the remainder
        println!("Result: {}", result);

        let final_result = random_number | result; // Bitwise OR with the random number
        println!("Random Number: {}, Result: {}, Final Result: {}",random_number, result, final_result);
}
