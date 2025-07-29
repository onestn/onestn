use std::collections::HashSet;
// BtreeSet과의 차이는 순서뿐

fn main() {
    let many_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 10];

    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    
    println!("There are {} unique numbers, so we are missing {}.", hashset_length, 10 - hashset_length);

    let mut missing_vec = vec![];
    for number in 0..10 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    println!("It does not contain: ");
    for number in missing_vec {
        println!("  - {number}");
    }
}
