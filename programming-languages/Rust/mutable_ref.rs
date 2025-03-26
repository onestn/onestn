// 여기서 벡터를 참조로 취합니다(&).
// 요소를 변경하는 것은 허용되지 않습니다.
// 소유권을 취하지 않고 빌리기만 합니다.
fn print_vec(numbers: &Vec<i32>) {
   for number in numbers {
       print!("{} ", number);
   }
   println!()
}


// 여기서 벡터를 가변 참조로 취합니다(&mut).
// 요소와 벡터 자체를 변경하는 것은 허용되지 않습니다.
// 여전히 소유권을 취하지 않고 빌리기만 합니다.
fn add_one(numbers: &mut Vec<i32>) {
   numbers.push(1)
}


fn main() {
   let mut numbers = vec![1,1,1];
   // 참조를 전달합니다.
   print_vec(&numbers);
   // 가변 참조를 전달합니다.
   add_one(&mut numbers);
   // 참조를 다시 전달합니다.
   print_vec(&numbers);
}
