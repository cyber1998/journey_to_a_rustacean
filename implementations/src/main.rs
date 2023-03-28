struct Polygon {
    number_of_sides: i16,
    name: String
}

impl Polygon {
    pub fn get_details(&self) {
        println!("I am a {} with {} sides!", self.name, self.number_of_sides);
    }
}

fn main() {
    let square:Polygon = Polygon{number_of_sides: 4, name: String::from("Square")};
    let rectangle:Polygon = Polygon{number_of_sides: 4, name: String::from("Rectangle")};

    square.get_details();
    rectangle.get_details();
}