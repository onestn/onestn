use std::time::Instant;
use std::thread;
use std::time::Duration;

fn main() {
    let start = Instant::now();

    thread::sleep(Duration::from_secs(2));

    let duration = start.elapsed();
    let total_secs = duration.as_secs();

    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    println!("Total Time: {:02}:{:02}:{:02}", hours, minutes, seconds);
}
