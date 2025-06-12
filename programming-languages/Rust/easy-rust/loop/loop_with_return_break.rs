fn main() {
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter; // loop를 중단하고 counter를 반환 
        }
    };

    println!("{my_number}");
}
