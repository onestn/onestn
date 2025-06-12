fn main() {
    for number in 0..3 {
        println!("The number is: {}", number);
    }

    for number in 0..=3 {
        println!("The next number is: {}", number);
    }

    for _ in 0..3 {
        println!("Printing the same thing three times");
    }
}
