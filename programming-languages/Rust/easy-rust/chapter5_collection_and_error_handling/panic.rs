fn print_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("vector param must always have three items")
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    let my_vec = vec![8, 9, 10];
    print_three_things(my_vec);
}
