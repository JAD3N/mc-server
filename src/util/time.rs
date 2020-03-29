use std::time::Duration;
use std::thread;

pub fn get_millis() -> u64 {
    chrono::Local::now().timestamp_millis() as u64
}

pub fn get_nanos() -> u64 {
    chrono::Local::now().timestamp_nanos() as u64
}

pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}