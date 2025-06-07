fn main() {
    let array1 = ["One", "Two"]; // [&str; 2]
    let array2 = ["One", "Two", "Five"]; // [&str; 3]
    let my_array = ["a"; 10];
    println!("{my_array:?}");

    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]); // 10
    
    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!(
        "Three to five: {:?},
start at two: {:?},
end at five: {:?},
everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything);

    // 마지막 숫자도 포함시키려면 ..에 =를 추가합니다.
    println!("{:?}", &array_of_ten[2..=5]);
}
