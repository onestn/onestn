use std::fmt::Debug;

fn print_number<T: Debug>(number: T) {
    println!("Here is your number: {:?}", number);
}

fn main() {
    print_number(5);
}
