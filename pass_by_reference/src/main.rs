struct Car {
    brand: String
}



fn main() {
    let car: Car = Car {brand: String::from("Mercedes")};
    show_info(&car);
}


fn show_info(car: &Car) {
    println!("Here's a {} for you", car.brand);
}