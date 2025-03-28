use chrono::{DateTime, Local};

fn main() {
    println!("Hello, world!");
    let local: DateTime = Local::now();
    println!("Today is {}", local.format("%A"));
}
