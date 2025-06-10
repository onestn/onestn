fn match_number(input: i32) {
    match input {
        number @ 4 => println!(
            "{number} is an unlucky number in China!"
        ),
        number @ 13 => println!(
            "{number} is unlucky in North America, lucky in Italy! In bocca al lupo!"
        ),
        _ => println!("Looks like a normal number"),
    }
}

fn main() {
    match_number(50);
    match_number(13);
    match_number(4);
}
