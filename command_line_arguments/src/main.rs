
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 0th index is the path of the compiled binary
    let first_name: &String = &args[1]; 
    let last_name: &String = &args[2]; 
    println!("Your name is {} {}", first_name, last_name);
}
