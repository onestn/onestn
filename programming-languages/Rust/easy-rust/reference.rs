fn main() {
    let country = String::from("Korea");
    let ref_one = &country;
    let ref_two = &country;

    println!("{}", ref_one);
}
