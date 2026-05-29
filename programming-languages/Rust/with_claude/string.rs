fn first_word(s: &str) -> String {
    s.split_whitespace().next().unwrap().to_string()
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("{sentence}");
    println!("{word}");
}
