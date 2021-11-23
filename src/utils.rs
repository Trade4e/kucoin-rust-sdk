use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time_as_millis() -> u128 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    since_the_epoch.as_millis()
}