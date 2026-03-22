mod database {
    pub mod connection {
        pub fn connect() {
            println!("DB 연결!");
        }
    }

    pub mod query {
        pub fn execute(sql: &str) {
            println!("실행: {sql}");
        }
    }
}

// 여기에 use 구문 작성
use database::connection;
use database::query;

fn main() {
    connection::connect();
    query::execute("SELECT * FROM users");
}
