use std::time::Instant;

pub struct TimeKeeper {
    pub start_time: Instant,
    pub time_threshold_seconds: u64,
}

impl TimeKeeper {
    pub fn is_time_over(&self) -> bool {
        let diff = self.start_time.elapsed();
        diff.as_secs() >= self.time_threshold_seconds
    }
}
