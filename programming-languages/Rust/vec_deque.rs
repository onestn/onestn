use std::collections::VecDeque;

fn main() {
    let mut deq = VecDeque::from([1, 2, 3]);
    println!("{}", deq.pop_front().unwrap());
}
