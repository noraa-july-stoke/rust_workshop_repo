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
    // Function to create a Person instance
    fn create_person(name: String, age: u32, gender: Gender) -> Person {
        Person { name, age, gender }
    }
    // Activity 1: Compound Types
    // Step 1: Declare the Person struct
    struct Person {
        name: String,
        age: u32,
        gender: Gender,
    }

    // Step 2: Declare the Gender enum
    enum Gender {
        Male,
        Female,
        NonBinary,
        Other(String),
    }

    // Step 3: Create an instance of the Person struct
    let person1 = create_person(String::from("John"), 25, Gender::Male);

    // Step 4: Print the details of person1
    println!("Person Details:");
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Gender: {}", person1.gender);

    // Step 5: Create another instance of the Person struct
    let person2 = create_person(String::from("Jane"), 30, Gender::Female);

    // Step 6: Print the details of person2
    println!("Person Details:");
    println!("Name: {}", person2.name);
    println!("Age: {}", person2.age);
    println!("Gender: {}", person2.gender);

    // Step 7: Create an instance of the Person struct with NonBinary gender
    let person3 = create_person(String::from("Alex"), 40, Gender::NonBinary);

    // Step 8: Print the details of person3
    println!("Person Details:");
    println!("Name: {}", person3.name);
    println!("Age: {}", person3.age);
    println!("Gender: {}", person3.gender);

    // Step 9: Create an instance of the Person struct with Other gender ("string")
    let person4 = create_person(
        String::from("Taylor"),
        35,
        Gender::Other(String::from("None of your business")),
    );

    // Step 10: Print the details of person4
    println!("Person Details:");
    println!("Name: {}", person4.name);
    println!("Age: {}", person4.age);
    println!("Gender: {}", person4.gender);

    // Activity 2: Tuples

    // Step 1: Declare the tuple
    let tuple1: (i32, f64, String) = (5, 5.0, String::from("Hello, world!"));

    // Step 2: Print the value of tuple1
    println!("tuple1 = {:?}", tuple1);
}
