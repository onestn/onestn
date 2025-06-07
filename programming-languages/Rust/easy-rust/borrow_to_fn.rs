fn print_country(country_name: &String) {
    // 3. 전달받은 참조의 값을 출력
    println!("{country_name}");
}

fn add_seoul(country_name: &mut String) {
    country_name.push_str("-Seoul");
    println!("Now it say: {country_name}");
}

fn adds_seoul(mut country_name: String) {
    country_name.push_str("-Seoul");
    println!("{country_name}");
}

fn main() {
    // 1. String 생성
    let country = String::from("Korea");
    // 2. 함수에 country 참조 전달
    print_country(&country);
    print_country(&country);

    let mut country = String::from("Korea");

    add_seoul(&mut country);

    print_country(&country);

    adds_seoul(country);

    print_country(&country); // Error.
}
