use std::time::Duration;

#[derive(Debug)]
pub struct Config {
    pub num_threads: usize,
    pub timeout_duration: Duration,
    pub max_retries: usize,
}

impl Config {
    // Constructor to create a new Config
    pub fn new(num_threads: usize, timeout_duration_secs: u64, max_retries: usize) -> Self {
        Config {
            num_threads,
            timeout_duration: Duration::from_secs(timeout_duration_secs),
            max_retries,
        }
    }
}
