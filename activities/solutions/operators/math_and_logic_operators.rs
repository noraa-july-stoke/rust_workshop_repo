fn main() {
    // Activity 1: Rectangle Calculation
    let length = 5;
    let width = 5;

    let area = length * width;
    let perimeter = 2 * (length + width);

    println!("Rectangle Area: {}", area);
    println!("Rectangle Perimeter: {}", perimeter);

    if length == width {
        println!("The rectangle is a square.");
    } else {
        println!("The rectangle is not a square.");
    }

    // Activity 2: Number Manipulation

    // Step 1: Generate a random number
    let random_number = generate_random_number();
    println!("Random Number: {}", random_number);

    // Step 2: Perform some calculations or operations
    let squared_number = random_number * random_number;
    println!("Squared Number: {}", squared_number);

    let modulo_result = random_number % 10;
    println!("Modulo Result: {}", modulo_result);

    // Step 3: Print the results
    println!("Random Number: {}", random_number);
    println!("Squared Number: {}", squared_number);
    println!("Modulo Result: {}", modulo_result);

    // Activity 3: String Concatenation
    let first_name = "John";
    let last_name = "Doe";

    let full_name = first_name.to_string() + " " + last_name;

    println!("Full Name: {}", full_name);
}
