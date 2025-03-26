fn main() {
    // let tup1 = (0, 0.1, "hello");
    // let tup2: (i32, f64, &str) = (1, 1.01, "bye");
    //
    // let (_, y, _) = tup2;
    //
    // println!("tup1 has {:?} and the value of y is: {}", tup1, y);

    let tup1 = (0, 0.1, ("hello", "world"));

    println!("{} {}", tup1.2 .0, tup1.2 .1);
}
