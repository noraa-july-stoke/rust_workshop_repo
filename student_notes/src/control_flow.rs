#![allow(dead_code)]
/*
Control Flow In Rust
Here are most of the general constructs that fall under the category of control flow:

1. "if/else" Expressions:
    - if/else statements are used to conditionally execute code based on a boolean condition.
      They allow you to branch the execution flow based on whether a condition is true or false.
    - if/else statements are expressions in Rust, which means they evaluate to a value.
    - As with most other programming languages you can have as many else if statements as you want.
      The condition must evaluate to a boolean. You can also have as many else..if statements as
      you want. The else statement is optional. Lastly, you may omit the else..if if you only
      have one condition. For example: For example: if condition { 5 } else { 6 }.
*/

/*
2. "match" Expressions:
    - match expressions provide a powerful way to handle multiple patterns and make decisions
      based on them. They allow you to match a value against different patterns and execute
      code based on the matching pattern.
    - if you have a situation where you have a bunch of if/else statements that are checking
      the same variable for different values, the match statement is going to be your friend.
    - They are similar to switch/case statements in other languages.
*/

/*
3. "loop" Expressions:
    - loop statements create an infinite loop that continues until explicitly terminated.
      You can use break statements to exit the loop.
*/

/*
4. "while" Expressions:
    - while statements create a loop that executes the body of the loop while the condition
      is true. The condition is evaluated before each iteration of the loop.
      You can use break statements to exit the loop.
*/

/*
5. "for" Expressions:
    - for statements create a loop that executes the body of the loop for each item in a
      collection. The loop can be exited early using break statements.
*/

/*
6. "if let" Expressions:
    - if let statements are a more concise way to handle pattern matching that only cares
      about one specific pattern. They are useful for handling optional values. These are
      a more advanced topic and we will not cover them here, but here is a general
      idea of the syntax so you have an awareness of them:

*/

/*
7. "continue" Expressions:
    - continue statements are used to skip the rest of the current iteration of a loop and
      continue to the next iteration. They are useful for skipping over certain values in
      a collection.

8. "break" Expressions:
    - break statements are used to exit a loop early when a certain condition is met.. They
      are useful for exiting a loop when a certain condition is met.

9. "return" Expressions:
    - return statements are used to exit a function or closure early when a certain condition
      is met. If the return statement is inside a loop it will also terminate the loop and
      return the value from the function.
*/

pub fn main() {}
