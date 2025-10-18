#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

fn get_args() -> Args {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Wonseok Yang <onestone.yang@gmail.com>")
        .about("Rust version of `head`")
        .get_matches();

    Args {
        files: ...
        lines: ...
        bytes: ...
    }
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
