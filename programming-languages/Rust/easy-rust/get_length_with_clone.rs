// fn get_length(input: String) {
//     println!("It's {}", input.split_whitespace().count());
// }

fn get_length(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn main() {
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");

        // 매번 복제본을 제공합니다.
        // get_length(my_string.clone());
        
        // 이렇게 하면 50번의 반복 중에 단 한번도 복제가 발생하지 않습니다.
        get_length(&my_string);
    }
}
