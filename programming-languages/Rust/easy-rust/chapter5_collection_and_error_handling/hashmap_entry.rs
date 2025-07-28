use std::collections::HashMap;

fn main() {
    let book_collection = vec!["L Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap
            .entry(book)
            .or_insert(0);

        *return_value += 1;
    }

    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }
}
