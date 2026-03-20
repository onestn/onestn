fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None
    }
    Some(a / b)
}

fn chain_divide(a: f64, b: f64, c: f64) -> Option<f64> {
    if let Some(value) = safe_divide(a, b) {
        return safe_divide(value, c)
    }
    None
}

fn main() {
    println!("{:?}", chain_divide(100.0, 5.0, 2.0)); // Some(10.0)
    println!("{:?}", chain_divide(100.0, 0.0, 2.0)); // None
    println!("{:?}", chain_divide(100.0, 5.0, 0.0)); // None
}
