use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // 1. 파일 읽기
    let mut input_file = File::open("test.txt")?;
    let mut contents = String::new();

    input_file.read_to_string(&mut contents)?;

    println!("읽은 내용: {contents}");

    //2. 파일 쓰기
    let mut output_file = File::create("output.txt")?;
    output_file.write_all(b"test")?;

    println!("output.txt 파일에 'test'를 썼습니다.");

    Ok(())
}
