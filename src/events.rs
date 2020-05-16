use event_bus::{Event, EventBus};
use std::sync::Mutex;

lazy_static! {
    static ref MAIN: Mutex<Option<EventBus>> = Mutex::new(None);
}

pub fn init() {
    *MAIN.lock().unwrap() = Some(EventBus::new("main"));

    // log that events have been set up
    info!("init -> Events initialized.");
}