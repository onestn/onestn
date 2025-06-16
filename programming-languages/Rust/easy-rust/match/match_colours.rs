// fn match_colours(rbg: (i32, i32, i32)) {
//     match rbg {
//         (r, _, _) if r < 10 => println!("Not much red"),
//         (_, b, _) if b < 10 => println!("Not much blue"),
//         (_, _, g) if g < 10 => println!("Not much green"),
//         _ => println!("Each colour has at least 10"),
//     }
// }
fn match_colours(rbg: (i32, i32, i32)) {
    println!("Comparing a colour with {} red, {} blue, and {} green:", rbg.0, rbg.1, rbg.2);
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];
    let mut all_have_at_least_10 = true;
    //TODO:
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}
