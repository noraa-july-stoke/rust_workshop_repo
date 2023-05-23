fn main() {
        // Task 1: Check if a number is positive, negative, or zero
    let number = 42;

    if number > 0 {
        println!("The number is positive.");
    } else if number < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }

    // Task 2: Check if a person is old enough to vote
    let age = 20;

    if age >= 18 {
        println!("You are eligible to vote.");
    } else {
        println!("You are not yet eligible to vote.");
    }
    // Task 3: Check eligibility for a discount
    let is_student = true;
    let age = 22;

    if is_student && age <= 25 {
        println!("You are eligible for a student discount.");
    } else if !is_student && age >= 65 {
        println!("You are eligible for a senior discount.");
    } else {
        println!("You are not eligible for any discount.");
    }

    // Task 4: Check username validity
    let username = "myusername";

    if username.len() >= 8 {
        println!("Username is valid.");
    } else {
        println!("Username is too short.");
    }
}
