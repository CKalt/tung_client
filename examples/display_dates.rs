extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

fn display_timestamp(secs: u64) {
    let d = UNIX_EPOCH + Duration::from_secs(secs);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!{"{}",timestamp_str};
}

fn display_microtimestamp(micros: u64) {
    let d = UNIX_EPOCH + Duration::from_micros(micros);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!{"{}",timestamp_str};
}

fn main(){
    display_timestamp(1624760846);
    display_microtimestamp(1624760846331000);
}
