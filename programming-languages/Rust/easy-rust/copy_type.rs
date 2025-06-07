fn prints_number(number: i32) {
    println!("{number}");
}

fn prints_country(country_name: String) {
    println!("{country_name}");
}


fn main() {
    let my_number = 8;

    prints_number(my_number);

    // copy type이 아닌 경우 소유권 문제로 실행할 수 없지만,
    // my_number가 정수이기 때문에 자동 copy되어 소유권이 만료되지 않아 실행할 수 있습니다.
    prints_number(my_number);

    let country = String::from("Kiribati");
    prints_country(country.clone()); // 하지만 clone()을 통해 값을 복제하면 소유권이 만료되지
                                     // 않습니다.

    // String은 copy type이 아니기 때문에 소유권이 만료되어 더 이상 실행할 수 없습니다.
    prints_country(country);
}
