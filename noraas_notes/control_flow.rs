/*
Welcome to the control flow activity!
This activity is designed to introduce you to control flow in Rust.

Here are most of the general constructs that fall under the category of control flow:

1. "if/else" Expressions:
    - if/else statements are used to conditionally execute code based on a boolean condition.
      They allow you to branch the execution flow based on whether a condition is true or false.

2. "match" Expressions:
    - match expressions provide a powerful way to handle multiple patterns and make decisions
      based on them. They allow you to match a value against different patterns and execute
      code based on the matching pattern.
    - if you have a situation where you have a bunch of if/else statements that are checking
      the same variable for different values, the match statement is going to be your friend.


3. "loop" Expressions:
    - loop  statements create an infinite loop that continues until explicitly terminated.
      You can use break statements to exit the loop.

4. "while" Expressions:
    - while statements create a loop that executes the body of the loop while the condition
      is true. The condition is evaluated before each iteration of the loop.
      You can use break statements to exit the loop.

5. "for" Expressions:
    - for statements create a loop that executes the body of the loop for each item in a
      collection. The loop can be exited early using break statements.

6. "if let" Expressions:
    - if let statements are a more concise way to handle pattern matching that only cares
      about one specific pattern. They are useful for handling optional values. These are
      a more advanced topic and we will not cover them here, but here is a general
      idea of the syntax so you have an awareness of them:

    fn main() {
        let maybe_number: Option<i32> = Some(42);

        if let Some(number) = maybe_number {
            println!("The number is: {}", number);
        } else {
            println!("No number provided.");
        }
    }

7. "continue" Expressions:
    - continue statements are used to skip the rest of the current iteration of a loop and
      continue to the next iteration. They are useful for skipping over certain values in
      a collection.

8. "break" Expressions:
    - break statements are used to exit a loop early when a certain condition is met.. They
      are useful for exiting a loop when a certain condition is met.

9. "return" Expressions:
    - return statements are used to exit a function or closure early when a certain condition
      is met. They are useful for exiting a function when a certain condition is met. If the
      return statement is inside a loop it will also terminate the loop and return the value
      from the function.
 */

fn main() {
    /*
    Let's start off with basic if.. else statements. They work pretty much identically to
    the if/else statements in most other programming languages that have them.
    The general syntax is as follows:

    if condition1 {
        // Some logic
    } else if condition2 {
        // Some logic
    } else {
        // Some logic
    }

    As with most other programming languages you can have as many else if statements as you want.
    The condition must evaluate to a boolean. You can also have as many else..if statements as
    you want. The else statement is optional. Lastly, you may omit the else..if if you only
    have one condition. For example: For example: if condition { 5 } else { 6 }. Okay, let's
    code out our first if..else statement!
    */

    // 2. if/else statement example
    let number = 10;

    if number > 0 {
        println!("The number is positive");
    } else if number < 0 {
        println!("The number is negative");
    } else {
        println!("The number is zero");
    }

    /*
    Now that we have covered the if..else statements, let's move on to match expressions.
    Match expressions provide a powerful way to handle multiple patterns and make decisions
    based on them. They allow you to match a value against different patterns and execute
    code based on the matching pattern. They are similar to switch/case statements in
    javascript. The general syntax is as follows:
    */

    // 3. match expression example
    let day = "Wednesday";

    match day {
        "Monday" => println!("It's Monday"),
        "Tuesday" => println!("It's Tuesday"),
        "Wednesday" => println!("It's Wednesday"),
        "Thursday" => println!("It's Thursday"),
        "Friday" => println!("It's Friday"),
        _ => println!("It's a weekend day"),
    }

    // Another match statement example, note the syntax for including conditions.
    let data: bool = true;

    match data {
        _ if data == true => println!("Data is a boolean."),
        _ if data == false => println!("Data is a boolean."),
        _ => println!("Unknown data type."),
    }

    /*
    Now let's move on to loops. We have two types of loops in Rust: loop and while loops.
    The loop keyword creates an infinite loop that continues until explicitly terminated.
    You can use break statements to exit the loop. While loops execute a block of code while
    a certain condition is true. The condition is evaluated before each iteration of the loop.
    You can use break statements to exit the loop.
    */

    // 4. loop statement example
    let mut counter = 0;

    loop {
        println!("Loop iteration: {}", counter);
        counter += 1;

        if counter >= 5 {
            break;
        }
    }

    // 5. while loop statement example
    let mut number = 3;

    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }

    /*
    These were some examples of basic control flow constructs in Rust. Feel free to
    experiment with different conditions, patterns, and loop conditions to get a better
    understanding of how they work.
    */
}
