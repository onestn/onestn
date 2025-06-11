enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{}", description),
        ThingsInTheSky::Stars(n) => println!("{}", n),
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);

    check_skystate(&skystate);
}
