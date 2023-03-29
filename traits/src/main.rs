// Traits are to rust are to what interfaces are to Java
// Or it is similiar to implementing magic methods in Python


struct Laptop {
    ram: u8,
    cpu: f32,
    cpu_brand: String,
    ram_brand: String
}

// toString trait a built in trait in Rust
impl ToString for Laptop {
    fn to_string(&self) -> String {
       return format!("{} {} processor with {} {} RAM", self.cpu_brand, self.cpu, self.ram_brand, self.ram); 
    }
}

// Custom trait
trait IsGamingPC {
    fn has_enough_ram(&self) -> bool;
    fn has_enough_cpu(&self) -> bool;
    fn fit_or_unfit(&self) -> String;

}

impl IsGamingPC for Laptop {
    fn has_enough_ram(&self) -> bool {
        if self.ram >= 16 {
            return true;
        }
        return false;
    }

    fn has_enough_cpu(&self) -> bool {
        if self.cpu >= 3.8 {
            return true;
        }
        return false;
    }

    fn fit_or_unfit(&self) -> String {
        if self.has_enough_cpu() && self.has_enough_ram() {
            return String::from("fit");
        }
        return String::from("unfit");
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
    println!("This laptop is {} for gaming!", laptop.fit_or_unfit());
}
