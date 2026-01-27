use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?.to_string()
        .parse::<u32>()?.to_string()
        .parse::<i32>()?;
    Ok(parsed_number)
}

fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }
}
