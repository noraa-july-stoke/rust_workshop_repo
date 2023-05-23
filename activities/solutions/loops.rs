fn main() {
    // 1. Loop from ten to one and print "Liftoff! 🚀" at the end
    let mut count = 10;

    loop {
        println!("{}", count);
        count -= 1;

        if count == 0 {
            println!("Liftoff! 🚀");
            break;
        }
    }

    // 2. FizzBuzz using a while loop
    let mut number = 1;

    while number <= 50 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }

        number += 1;
    }

    // 3. Loop using a for loop
    let favorite_foods = ["pizza", "sushi", "chocolate", "ice cream", "tacos"];

    for food in favorite_foods.iter() {
        println!("I love {}!", food);
    }
}
