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
