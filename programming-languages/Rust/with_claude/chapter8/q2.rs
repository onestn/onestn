fn make_even(numbers: &mut Vec<i32>) {
    fn is_odd(number: i32) -> bool {
        if number % 2 != 0 {
            return true
        }
        false
    }
    for number in numbers {
        if is_odd(*number) {
            *number += 1;
        }
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    make_even(&mut nums);
    println!("{:?}", nums);
}
