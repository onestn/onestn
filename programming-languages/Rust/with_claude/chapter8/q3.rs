fn count_vowels(s: &str) -> usize {
    // let mut count = 0;
    // for c in s.chars() {
    //     if matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
    //         count += 1;
    //     }
    // }
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

fn main() {
    println!("{}", count_vowels("Hello World")); // 3
    println!("{}", count_vowels("AEIOU aeiou")); // 10
    println!("{}", count_vowels("rhythm"));      // 0
}
