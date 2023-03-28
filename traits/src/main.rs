// Traits are to rust are to what interfaces are to Java
// Or it is similiar to implementing magic methods in Python


struct Laptop {
    ram: u8,
    cpu: f32,
    cpu_brand: String,
    ram_brand: String
}

// toString trait
impl ToString for Laptop {
    fn to_string(&self) -> String {
       return format!("{} {} processor with {} {} RAM", self.cpu_brand, self.cpu, self.ram_brand, self.ram); 
    }
}

fn main() {
    let laptop: Laptop = Laptop {
        ram: 16,
        cpu: 4.5,
        cpu_brand: String::from("Intel"),
        ram_brand: String::from("GSKILL")
    };
    println!("{}", laptop.to_string());
}
