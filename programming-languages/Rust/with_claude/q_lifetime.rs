struct Config {
    host: String,
    port: u32,
}

fn create_config() -> Config {
    let host = String::from("localhost");
    Config {
        host: host,
        port: 8080,
    }
}

fn main() {
    create_config();
}
