fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10; // *를 사용해서 i32 값을 변경
                    // *를 사용하는 것을 역참조라고 합니다.
    println!("{}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Second_number = triple_reference? {}",
        second_number == ***triple_reference);
}
