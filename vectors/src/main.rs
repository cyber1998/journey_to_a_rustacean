
fn main() {
    let mut fruits = vec!["Apple", "Banana"];
    fruits.push("Citrus");

    for fruit in fruits.iter() {
        println!("{}", fruit);
    }

    fruits.remove(2);
    println!("{:?}", fruits);
}
