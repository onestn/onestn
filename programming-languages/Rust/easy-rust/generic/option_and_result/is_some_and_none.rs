fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];

    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap()); // 앞서 값을 확인했기 때문에 unwrap()을
                                                            // 사용해도 안전합니다.
        } else {
            println!("We got nothing.");
        }
    }
}
