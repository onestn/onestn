enum FileError {
    NotFound(String),
    PermissionDenied(String),
}

enum FileResult {
    Ok(String),
    Err(FileError),
}

impl FileResult {
    fn report(&self) -> String {
        match self {
            FileResult::Ok(content) 
                => format!("성공: {}바이트 읽음", content.len()),
            FileResult::Err(FileError::NotFound(path)) 
                => format!("에러: {path} 파일을 찾을 수 없음"),
            FileResult::Err(FileError::PermissionDenied(path)) 
                => format!("에러: {path} 접근 권한 없음"),
        }
    }
}

fn main() {
    let results = vec![
        FileResult::Ok(String::from("file content here")),
        FileResult::Err(FileError::NotFound(String::from("/tmp/data.csv"))),
        FileResult::Err(FileError::PermissionDenied(String::from("/etc/secret"))),
    ];

    for result in &results {
        println!("{}", result.report());
    }
}
