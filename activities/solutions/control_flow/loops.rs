pub fn main() {
    // 1. loop that counts down from 10 to 1
    let mut countdown = 10;
    loop {
        println!("{}", countdown);
        countdown -= 1;
        if countdown == 0 {
            println!("Liftoff! 🚀");
            break;
        }
    }

    // 2. Fizzbuzz with while loop
    let mut number = 1;
    while number <= 50 {
        match (number % 3, number % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", number),
        }
        number += 1;
    }

    // 3. For loop over favorite foods
    let favorite_foods = ["Pizza", "Sushi", "Chocolate", "Ice Cream", "Tacos"];
    for food in &favorite_foods {
        println!("{}", food);
    }
}
