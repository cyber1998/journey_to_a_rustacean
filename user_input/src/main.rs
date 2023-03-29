fn main() {
    println!("This program will take in two numbers from user input and add them!");

    println!("Enter the first number: ");
    let mut input: String = String::new();
   
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let num1:i32 = input.trim().parse::<i32>().unwrap();
    
    input.clear();
    println!("Enter the second number: ");

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2:i32 = input.trim().parse::<i32>().unwrap();

    println!("The sum of two numbers is {}", (num1+num2));
    
}
