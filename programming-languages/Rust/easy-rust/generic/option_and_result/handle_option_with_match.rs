fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a None!"),
        }
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new();

    option_vec.push(take_fifth(new_vec));    // [None]
    option_vec.push(take_fifth(bigger_vec)); // [None, Some(5)]
    
    handle_option(option_vec);
}
