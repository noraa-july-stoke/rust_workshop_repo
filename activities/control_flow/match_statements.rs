use std::any::type_name;

/*
Welcome to the match expressions activity! In this activity, you will practice using match expressions in Rust.
Complete the tasks below in the main function to finish the activity. If you get stuck or need a hint,
refer to the examples in the notes, the Rust documentation, or ask for assistance. Good luck!

Task 1:
Write a match expression to determine the weather condition.
- Declare a variable named "weather" and assign it any weather condition of your choice, such as "sunny" or "rainy".
- Write a match expression that matches the value of "weather" and performs specific actions based on the value.
- If "weather" is "sunny", print "It's a sunny day!"
- If "weather" is "rainy", print "Remember to take an umbrella."
- If "weather" is "cloudy", print "Expect a partly cloudy day."
- For any other value of "weather", print "Unknown weather condition."
- Run your program and observe the output based on the value of "weather."

Task 2:
Write a match expression to determine the category of a product based on its price.
- Declare a variable named "price" and assign it any positive integer value of your choice.
- Write a match expression that matches the value of "price" and determines the category.
- If "price" is less than 100, print "Low-priced product."
- If "price" is between 100 and 500 (inclusive), print "Mid-priced product."
- If "price" is greater than 500, print "High-priced product."
- Run your program and observe the output based on the value of "price."

Task 3:
Write a match expression to determine the day of the week based on a number.
- Declare a variable named "day" and assign it any integer value from 1 to 7.
- Write a match expression that matches the value of "day" and determines the corresponding day of the week.
- If "day" is 1, print "Sunday."
- If "day" is 2, print "Monday."
- If "day" is 3, print "Tuesday."
- If "day" is 4, print "Wednesday."
- If "day" is 5, print "Thursday."
- If "day" is 6, print "Friday."
- If "day" is 7, print "Saturday."
- For any other value of "day", print "Invalid day."
- Run your program and observe the output based on the value of "day."

Task 4:
Write a match expression to determine the animal sound.
- Declare a variable named "animal" and assign it any animal name of your choice, such as "cat" or "dog".
- Write a match expression that matches the value of "animal" and performs specific actions based on the value.
- If "animal" is "cat", print "Meow!"
- If "animal" is "dog", print "Woof!"
- If "animal" is "bird", print "Tweet!"
- For any other value of "animal", print "Unknown animal sound."
- Run your program and observe the output based on the value of "animal."

Task 5:
Write a match expression to determine the type of a variable.
- Declare a variable named "data" and assign it any value of your choice, such as a number, string, or boolean.
- Write a match expression that matches the type of "data" and performs specific actions based on the type.
- If "data" is an integer, print "Data is an integer."
- If "data" is a string, print "Data is a string."
- If "data" is a boolean, print "Data is a boolean."
- For any other type of "data", print "Unknown data type."
- Run your program and observe the output based on the value and type of "data."


Task 6:
Write a match expression to determine the grade based on a student's score.
- Declare a variable named "score" and assign it any integer value of your choice within the range of 0 to 100.
- Write a match expression that matches the value of "score" and determines the corresponding grade.
- If "score" is between 90 and 100, print "Grade: A"
- If "score" is between 80 and 89, print "Grade: B"
- If "score" is between 70 and 79, print "Grade: C"
- If "score" is between 60 and 69, print "Grade: D"
- If "score" is below 60, print "Grade: F"
- Run your program and observe the output based on the value of "score."

Feel free to modify the code or add more match expressions to practice different patterns. Enjoy exploring match expressions!
*/


pub fn main() {
    // Task 1: Determine the weather condition
    let weather = "sunny";

    match weather {
        "sunny" => println!("It's a sunny day!"),
        "rainy" => println!("Remember to take an umbrella."),
        "cloudy" => println!("Expect a partly cloudy day."),
        _ => println!("Unknown weather condition."),
    }

    // Task 2: Determine the category of a product based on its price
    let price = 250;

    match price {
        0..=99 => println!("Low-priced product."),
        100..=500 => println!("Mid-priced product."),
        _ => println!("High-priced product."),
    }

    // Task 3: Determine the day of the week based on a number
    let day = 3;

    match day {
        1 => println!("Sunday."),
        2 => println!("Monday."),
        3 => println!("Tuesday."),
        4 => println!("Wednesday."),
        5 => println!("Thursday."),
        6 => println!("Friday."),
        7 => println!("Saturday."),
        _ => println!("Invalid day."),
    }

    // Task 4: Determine the animal sound
    let animal = "dog";

    match animal {
        "cat" => println!("Meow!"),
        "dog" => println!("Woof!"),
        "bird" => println!("Tweet!"),
        _ => println!("Unknown animal sound."),
    }

    // Task 5: Determine the type of a variable
    let data: bool = true;

    match data {
        _ if data == true => println!("Data is a boolean."),
        _ if data == false => println!("Data is a boolean."),
        _ => println!("Unknown data type."),
    }


    // Task 6: Determine the grade based on a student's score
    let score = 85;

    match score {
        90..=100 => println!("Grade: A"),
        80..=89 => println!("Grade: B"),
        70..=79 => println!("Grade: C"),
        60..=69 => println!("Grade: D"),
        _ => println!("Grade: F"),
    }
}
