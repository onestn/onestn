use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, i32> {
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        word_map.entry(word).or_insert(1);
        println!("{}", word);
    }

    word_map
}

fn main() {
    let text = "hello world hello rust hello world";
    let counts = word_count(text);

    for (word, count) in &counts {
        println!("{word}: {count}");
    }
}
