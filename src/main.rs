use chrono::Local;
use chrono_tz::{
    Europe::London,
    America::{Los_Angeles, New_York},
};

fn main() {
    let now = Local::now();

    println!("{0: <15} | {1: <10}", "City", "Time");
    println!("{0: <15} | {1: <10}", "_______________", "____________________________________");
    println!("{0: <15} | {1: <10}", "Local", now);
    println!("{0: <15} | {1: <10}", "New York", now.with_timezone(&New_York));
    println!("{0: <15} | {1: <10}", "Los Angeles", now.with_timezone(&Los_Angeles));
    println!("{0: <15} | {1: <10}", "London", now.with_timezone(&London));
}
