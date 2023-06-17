fn main(){
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

    // Task 5: Determine the grade based on a student's score
    let score = 85;

    match score {
        90..=100 => println!("Grade: A"),
        80..=89 => println!("Grade: B"),
        70..=79 => println!("Grade: C"),
        60..=69 => println!("Grade: D"),
        _ => println!("Grade: F"),
    }

    // Task 6: Determine the category of a value based on its numerical range
    let value = 42;
    match value {
        0 => println!("Value is zero!"),
        1 | 2 | 3 => println!("Value is a small number."),
        4..=10 => println!("Value is in the range of 4 to 10."),
        11..=50 => println!("Value is in the range of 11 to 50."),
        _ => println!("Value is something else."),
    }
}
