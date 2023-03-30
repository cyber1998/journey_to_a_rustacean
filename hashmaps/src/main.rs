use std::collections::HashMap;

fn main() {
    let nums: [i32; 7] = [1,1,2,2,3,3,4];
    let mut counter: HashMap::<i32, i32> = HashMap::new();
    for num in nums.iter() {
        if counter.contains_key(num) {
            counter.insert(*num, counter[num] + 1);
        }
        else {
            counter.insert(*num, 1);
        }
    }
    println!("{:?}", counter);
    
}
