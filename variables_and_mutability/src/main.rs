fn main() {
    /* Variables within a local scope is declared in snake case. Global constants are declared
    in capital case.
    */

    // Auto derived datatypes

    let cat = "Cat";
    let number = 5;
    let temperature = 16.8;
    let refrigerator_is_full = true;
    println!(
        "Cat: {}, Number: {}, Temperature: {}, Is the refrigerator full? {}",
        cat, number, temperature, refrigerator_is_full
    );

    // Manually declaring a variable and it's datatype is usually considered a good practice
    
    // There are two string types - &str and String
    let dog: &str = "Dog"; // This is a mutable string
    let monkey: String = String::from("Monkey"); // This is an immutable string

    println!("Dog: {}, Monkey: {}", dog, monkey);
    // Variables should be declared with mut to mutate them and change their values
    let mut mutable_number = 7;
    println!("The value of mutable_number at this moment is: {}", mutable_number);
    mutable_number = 8;
    println!("The value of mutable_number after changing is: {}", mutable_number);
    mutable_number = 9;
    println!("The value of mutable_number after changing again is: {}", mutable_number);







}
