fn main() {
    let mut number = 10;
    let number_change = &mut number;
    let number_ref = 
    *number_change += 10;
    
    let number_ref = &number;
    println!("{}", number_ref);
}
