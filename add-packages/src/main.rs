// can call with cargo add chrono
use chrono::{Local, Utc};

fn main() {
    let now = Utc::now();
    println!("Current time now in UTC: {}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Current time now in UTC, formatted: {}", formatted);

    let local = Local::now();
    println!("Local time now: {}", local);
}