fn main() {
    let mut fixed_size_array: [i32; 4] = [1, 2, 3, 4];

    // Compiler automatically deduces the datatype and size. However it is still a fixed
    // Size array
    let mut dynamic_array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Array with default values. value; times repeated
    let default_array: [i32; 10] = [5; 10];

    println!("{:?}", fixed_size_array);
    println!("{:?}", dynamic_array);
    println!("{:?}", default_array);
}
