fn main() {
    // Simple integer loops
    let lower_limit: i8 = 1;
    let upper_limit: i8 = 100;
    let range: core::ops::Range<i8> = lower_limit..upper_limit;
    for i in range {
        println!("{}", i);
    }

    // Simple string loops
    let birds = vec!["parrot", "chicken", "sparrow", "pigeon"];

    // We should use iter to prevent ownership of the values inside the array to
    // move to the for loop.
    for bird in birds.iter() {
        println!("{}", bird);
    }


    // Enumeration
    for (idx, bird) in birds.iter().enumerate() {
        println!("In the {} index, there is a {}", idx, bird);
    }
}
