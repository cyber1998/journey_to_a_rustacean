fn main() {
    let mut a: i8 = 10;
    let mut b: i8 = 32;
    let result1: i16 = add_by_reference(&a, &b);
    println!("{}", result1);

    a = 8;
    b = 1;
    let result2: i8 = add_by_value(a, b);
    println!("{}", result2);

    if result1 > result2.into() {
        println!("{} is a bigger number than {}", result1, result2);
    }
    else {
        println!("{} is a smaller number than {}", result1, result2);
    }
}


fn add_by_reference(a: &i8, b: &i8) -> i16 {
    return (*a + *b).into();
}

fn add_by_value(a: i8, b: i8) -> i8 {
    return a + b;
}