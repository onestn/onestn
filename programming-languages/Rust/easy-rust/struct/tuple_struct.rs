struct FileDirectory;

struct Colour(u8, u8, u8);

fn main() {
    let my_colour = Colour(50, 0, 50);
    println!("The second part of the colour is: {}", my_colour.1);
}


