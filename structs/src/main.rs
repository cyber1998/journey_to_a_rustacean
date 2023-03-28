struct Car {
    brand: String,
    model: String,
    body_type: String,
    price: f32

}


fn main() {
    let car_1:Car = Car {
        brand: String::from("BMW"),
        model: String::from("M3 2021"),
        body_type: String::from("Coupe"),
        price: 75000.0
    };

    println!(
        "My car is a {} {} which is an amazing {}. I bought it for ${} :) ",
         car_1.brand,
         car_1.model,
         car_1.body_type,
         car_1.price
        )
}
