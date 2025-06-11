fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("The counter is now: {counter}");

        if counter == 5 {
            break;
        }
    }
}
