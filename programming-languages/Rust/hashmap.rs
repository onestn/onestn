use std::collections::HashMap;

fn main() {
    // Rust's HashMap does not keep the insertion order.
    let mut songs = HashMap::from([
        ("Toto", "Africa"),
        ("Post Malone", "Rockstar"),
        ("twenty one pilots", "Stressed Out"),
    ]);

    println!("----- Playlists -----");

    if songs.contains_key("Toto") && songs.values().any(|&val| val == "Africa") {
        println!("Toto's africa is the best song!");
    }

    songs.insert("a-ha", "Take on Me"); // INSERT
    songs.entry("Post Malone").and_modify(|v| *v = "Happier"); // UPDATE
    
    for (artist, title) in songs.iter() {
        println!("{} - {}", artist, title);
    }

    println!("---------------");
    songs.remove("Post Malone"); // DELETE
    
    println!(
        "{:?}",
        songs
            .get("Post Malone")
            .unwrap_or(&"Post Malone is not in the playlist")
    );
}
