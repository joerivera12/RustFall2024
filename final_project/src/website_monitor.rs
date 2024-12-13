use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::config::Config;

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: u64, // UNIX timestamp (seconds since epoch)
}

// Function to check the status of a single website
pub fn check_website(url: String, timeout_duration: Duration) -> WebsiteStatus {
    let start = Instant::now();

    let result = match ureq::get(&url).timeout(timeout_duration).call() {
        Ok(response) => Ok(response.status()),
        Err(e) => Err(e.to_string()),
    };

    let response_time = start.elapsed();

    // Get the current time as a UNIX timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    WebsiteStatus {
        url,
        status: result,
        response_time,
        timestamp,
    }
}

// Function to handle monitoring of multiple websites concurrently
pub fn monitor_websites(config: &Config, websites: Vec<String>) -> Vec<WebsiteStatus> {
    let num_threads = config.num_threads;
    let timeout_duration = config.timeout_duration;

    // Divide the list of websites into chunks, one per thread
    let chunk_size = (websites.len() + num_threads - 1) / num_threads; // Round up
    let chunks: Vec<Vec<String>> = websites.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

    let mut handles = vec![];

    // Spawn threads for each chunk of websites
    for chunk in chunks {
        let timeout_duration = timeout_duration;

        let handle = thread::spawn(move || {
            let mut results = vec![];
            for url in chunk {
                results.push(check_website(url, timeout_duration));
            }
            results
        });

        handles.push(handle);
    }

    // Collect results from all threads
    let mut results = vec![];
    for handle in handles {
        results.extend(handle.join().expect("Thread panicked"));
    }

    results
}
