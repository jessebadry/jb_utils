use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct PerformanceTimer {
    start: u128,
}

impl PerformanceTimer {
    pub fn new() -> PerformanceTimer {
        Self::default()
    }
}

fn time_now_nanos() -> u128 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|e| panic!("Could not get time since epoch, why: {}", e))
        .as_nanos();

    time
}
impl Default for PerformanceTimer {
    fn default() -> Self {
        PerformanceTimer {
            start: time_now_nanos(),
        }
    }
}
impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        let time = time_now_nanos();
        let time_taken = time - self.start;
        let time_taken = (time_taken as f64) / 1000000.0;
        println!("Time taken in milliseconds: {}", time_taken);
    }
}
