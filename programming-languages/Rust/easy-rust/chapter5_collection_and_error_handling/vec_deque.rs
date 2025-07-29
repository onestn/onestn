use std::collections::VecDeque;
// VecDeque는 링 버퍼(Ring buffer)를 활용하여 효율적으로 동작한다.
// 일반적으로 Vec보다 약간 느리지만, 양쪽 끝에서 작업을 수행해야 할 때는 링 버퍼 덕에 작업이 훨씬
// 빠르다.

fn main() {
    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for _ in 0..600000 {
        my_vec.pop_front();
    }
}
