fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    for i in 0..10 {
        println!("fib({i}) = {}", fib(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_base_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }
}
