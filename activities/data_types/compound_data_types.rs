/*
Activity 1: Compound Types

1. Declare a struct named `Person` with the following fields:
   - `name` of type `String`
   - `age` of type `u32`
   - `gender` of type `Gender`

2. Declare an enum named `Gender` with the following variants:
   - `Male`
   - `Female`
   - `NonBinary`
   - `Other(String)`

   Note for next steps, use the create_person() function I have included to create instances of the Person struct.
   Also, the first argument passed to this function should use the String::from("string") method to create a new String.
   This is also going to be your first debugging experience, I have left out some details on purpose. You are
   to read the error message in the console, and fix the code to make it work. I promise that the answer is found
   in the error message.You will need to add a line of code to the Enum you created in order to make it print.
   You will also need to add something inside the '{}' in your println! macro to make it print. Note, there are
   two bugs in the code, you will fix one, then get another error message and fix the second one. If done correctly,
   the code will run on the third try.

3. Create an instance of the `Person` struct with the name "John", age 25, and the `Gender::Male` variant.
   Assign it to a variable named `person1`.

4. Print the `name`, `age`, and `gender` of `person1` using the `println!` macro.

5. Create another instance of the `Person` struct with the name "Jane", age 30, and the `Gender::Female` variant.
   Assign it to a variable named `person2`.

6. Print the `name`, `age`, and `gender` of `person2` using the `println!` macro.

7. Create an instance of the `Person` struct with the name "Alex", age 40, and the `Gender::NonBinary` variant.
   Assign it to a variable named `person3`.

8. Print the `name`, `age`, and `gender` of `person3` using the `println!` macro.

9. Create an instance of the `Person` struct with the name "Taylor", age 35, and the `Gender::Other` variant
   with the value "two-spirit" or "None of your business" or something else. Assign it to a variable named `person4`.

10. Print the `name`, `age`, and `gender` of `person4` using the `println!` macro.

Activity 2: Tuples

1. Declare a tuple named `tuple1` with the following values:
   - `5` of type `i32`
   - `5.0` of type `f64`
   - `"Hello, world!"` of type `String`

2. Print the value of `tuple1` using the `println!` macro.

Instructions:
1. Complete the activities as described above.
2. Run the program and observe the output.

Note: Remember to use appropriate variable names and proper formatting for the output.
*/

pub fn main() {
// Your code here.
}
