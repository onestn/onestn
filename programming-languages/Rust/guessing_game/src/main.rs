use std::io;
use rand::Rng; // Rng is a trait for random number generators
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // get_range()는 Rng 트레이트에 정의되어 있음
    // 1..=100은 1부터 100까지의 범위를 나타냄
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The seceret number is: {secret_number}");
    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
