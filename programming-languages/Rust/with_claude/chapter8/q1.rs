fn average(numbers: &Vec<i32>) -> Option<f64> {
    if numbers.is_empty() {
        return None
    }
        
    let mut sum: i32 = 0;
    for number in numbers {
        sum += number;
    }

    return Some((sum / (numbers.len() as i32)).into())
}

fn main() {
    let nums = vec![10, 20, 30, 40];
    let empty: Vec<i32> = Vec::new();

    println!("{:?}", average(&nums));  // Some(25.0)
    println!("{:?}", average(&empty)); // None
}
