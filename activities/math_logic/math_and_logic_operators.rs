use std::time::SystemTime;

/*
Welcome to the combined operators activity! In this activity, you will be applying the concepts of logic and math operators in
combination to solve various tasks. Complete the tasks below to finish the activity. If you get stuck or need a hint, feel
free to refer to the examples in the notes, the Rust documentation, or ask for assistance. Good luck!

Activity 1: Rectangle Calculation

1. Calculate the area and perimeter of a rectangle.
   - Declare two variables, `length` and `width`, and assign them with appropriate
     numeric values representing the length and width of the rectangle.
   - Calculate the area by multiplying `length` and `width` together and store
     the result in a variable called `area`.
   - Calculate the perimeter by adding twice the `length` and twice the `width`
     together and store the result in a variable called `perimeter`.
   - Print the calculated area and perimeter of the rectangle.

2. Determine if the rectangle is a square.
   - Use a comparison operator to check if `length` and `width` are equal.
   - If they are equal, print "The rectangle is a square."
   - If they are not equal, print "The rectangle is not a square."

Activity 2: Number Manipulation

In this activity, you will manipulate a random number generated using a custom function.
Perform calculations on the number and determine if it is even or odd. We don't have access
to the 'rand' crate so I wrote a function, `generate_random_number` that simulates a random
number generator.

1. Generate a random number between 1 and 100 (inclusive) using the `generate_random_number` function.
   - Add the random number to 50 and store the result in a variable called `sum`.
   - Subtract 25 from the `sum` and store the result in a variable called `result`.
   - Print the final result.

2. Determine if the result is even or odd.
   - Use the modulo operator `%` to check if the `result` is divisible by 2 without any remainder.
   - If the `result` is divisible by 2, print "Even number."
   - If the `result` is not divisible by 2, print "Odd number."

Instructions:
1. In the `main` function, call the `generate_random_number` function to
   generate a random number.
2. Add the random number to 50 and store the result in a variable called `sum`.
3. Subtract 25 from the `sum` and store the result in a variable called `result`.
4. Print the final result.
5. Use the modulo operator `%` to check if the `result` is divisible by 2.
6. If the `result` is divisible by 2, print "Even number."
7. If the `result` is not divisible by 2, print "Odd number."

Make sure to run your program and observe the output, which should display
the final result and whether it is an even or odd number.

Activity 3: String Concatenation

1. Declare two variables, `first_name` and `last_name`, and assign them with
   your first and last name, respectively.
2. Use the concatenation operator `+` to concatenate `first_name` and `last_name`
   together, separated by a space, and store the result in a variable called `full_name`.
3. Print the `full_name`.

Remember to use appropriate variable names and proper formatting for the output.
Run your program and observe the results based on the input values.

Feel free to modify the code or add more tasks to practice different combinations
of logic and math operators. Have fun exploring the combined operators in Rust!
*/

/*
    Function to generate a random number...
    Note... I wrote this function to simulate a random number generator since
    we don't have access to the 'rand' crate in this directory. This function
    is not meant to be used in production code. It is definitely not
    cryptographically secure, lol!

    Also, since I am using some things we haven't covered, I will explain
    what is going on here. The SystemTime::now() function returns a SystemTime
    struct. The SystemTime struct has a function called duration_since() that
    returns a Duration struct. The Duration struct has a function called
    as_millis() that returns the number of milliseconds since the UNIX epoch.
    The UNIX epoch is the number of milliseconds since January 1, 1970. This
    is a common way to represent time in computer systems. The as_millis()
    function returns a Result enum. The Result enum has an expect() function
    that will return the value of the Result if it is Ok or panic if it is
    Err. The expect() function takes a string as an argument that will be
    displayed if the Result is Err. The as_millis() function will return Err
    if the duration is negative. This can happen if the system time is set
    to a time before the UNIX epoch. This is unlikely to happen on modern
    systems but it is possible. If the duration is negative, the program
    will panic and display the message "Failed to retrieve system time."
    Then we convert the time to a u32 and assign it to the timestamp variable.
    Here is what we are doing to the timestamp after that...

   ^= (XOR bitwise assignment operator):

      The ^= operator performs the XOR bitwise operation between two operands and
      assigns the result to the left operand. In this case,
      xorshift_state ^= xorshift_state << 13; means taking the XOR of xorshift_state
      with the result of xorshift_state << 13 and assigning the result back to xorshift_state.
      The XOR operation sets each bit of the result to 1 if the corresponding bits of
      the two operands are different; otherwise, it sets the bit to 0.

   << (Left shift operator):
      The << operator shifts the bits of the left operand to the left by the number of
      positions specified by the right operand. In this case, xorshift_state << 13 means
      shifting the bits of xorshift_state 13 positions to the left. Shifting the bits to
      the left effectively multiplies the value by 2 raised to the power of the shift amount.

      Basically, we are getting a timestamp, and then manipulating it into a (very) pseudo
      random number!

*/

// Notice how rust doesn't even trust that system time is available?!
fn generate_random_number() -> u32 {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to retrieve system time.")
        .as_millis();

    let mut xorshift_state = timestamp as u32;
    xorshift_state ^= xorshift_state << 13;
    xorshift_state ^= xorshift_state >> 17;
    xorshift_state ^= xorshift_state << 5;

    xorshift_state % 100 + 1
}

pub fn main() {}
