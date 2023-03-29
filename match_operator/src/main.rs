fn main() {
    let choice: i32 = 34;

    match choice {
        0 | 1 => println!("Binary numbers? Really?"),
        2 => println!("Your choice is bad."),
        3 => println!("Yeah, better choice"),
        4..=100 => println!("Fantastic choice!"),
        _ => println!("Please choose a number between 0 and 100")
    };
}
