enum Command {
    Ping,
    Echo(String),
    Move { x: i32, y: i32 },
    Quit,
}

impl Command {
    fn execute(&self) {
        match self {
            Command::Ping => println!("PONG"),
            Command::Echo(msg) => println!("ECHO: {msg}"),
            Command::Move { x, y } => println!("이동: ({x}, {y})"),
            Command::Quit => println!("종료"),
        }
    }
}

fn main() {
    let cmds = vec![
        Command::Ping,
        Command::Echo(String::from("hello")),
        Command::Move { x: 10, y: -5 },
        Command::Quit,
    ];

    for cmd in &cmds {
        cmd.execute();
    }
}
