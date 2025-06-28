fn return_number<MyType>(number: Mytype) -> MyType {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
}
