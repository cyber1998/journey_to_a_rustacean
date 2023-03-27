enum Hobbies {
    Programming,
    Gaming,
    Driving,
    Travelling
}
// Enums can take in parameters too, and act as a function!
enum Details {
    Age(u8),
    Name(String),
    FavoriteHobby(Hobbies)
}

fn main() {
    let favorite_hobby: Hobbies = Hobbies::Programming;
    match favorite_hobby {
        Hobbies::Driving => driving(),
        Hobbies::Gaming => gaming(),
        Hobbies::Programming => programming(),
        Hobbies::Travelling => travelling(),
    }

    // Matching enum values.
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let age = Details::Age(25);
    let name = Details::Name(String::from("Cyber"));
    let hobby = Details::FavoriteHobby(Hobbies::Programming);

    match name {
        Details::Age(value) => println!("Value is {}", value),
        Details::Name(value) => println!("Name is {}", value),
        Details::FavoriteHobby(value) => programming(),
    }

    match age {
        Details::Age(value) => println!("Age is {}", value),
        Details::Name(value) => println!("Name is {}", value),
        Details::FavoriteHobby(value) => programming(),
    }

    match hobby {
        Details::Age(value) => println!("Value is {}", value),
        Details::Name(value) => println!("Name is {}", value),
        Details::FavoriteHobby(value) => programming(),
    }
}



fn driving() {
    println!("My favorite hobby is driving!");
}

fn programming() {
    println!("My favorite hobby is programming!");
}

fn gaming() {
    println!("My favorite hobby is gaming!");
}

fn travelling() {
    println!("My favorite hobby is travelling!");
}