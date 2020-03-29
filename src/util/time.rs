use std::time::{SystemTime, Duration};
use std::thread;

pub fn get_millis() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}