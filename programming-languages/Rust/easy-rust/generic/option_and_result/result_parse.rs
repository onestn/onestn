fn main() {
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];
    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);

        while let Some(information) = city.pop() {
            // 더 이상 꺼낼 수 없을 때까지 계속 진행
            // 벡터의 항목 수가 0개에 도달하면 None을 반환하고 멈춤
            if let Ok(number) = information.parse::<i32>() {
                // information 변수를 i32로 변경 시도
                // 반환 결과가 Ok(숫자)면 출력
                println!("The number is: {number}");
            }
            // 오류 발생 시 아무것도 하지 않음
            // 나머지는 모두 버림
        }
    }
}
