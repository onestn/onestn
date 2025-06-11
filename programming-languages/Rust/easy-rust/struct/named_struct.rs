struct Colour(u8, u8, u8);

struct SizeAndColour {
    size: u32,
    colour: Colour
}

fn main() {
    let my_colour = Colour(50, 0, 50);

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour
    };
}
