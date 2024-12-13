use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::thread;
use ureq;
use crate::config::Config;

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: u64, // Using a UNIX timestamp (seconds since epoch)
}

// Function to check the status of a single website
pub fn check_website(url: String, timeout_duration: Duration) -> WebsiteStatus {
    let start = Instant::now();
    
    let result = match ureq::get(&url).timeout(timeout_duration).call() {
        Ok(response) => Ok(response.status()),
        Err(e) => Err(e.to_string()),
    };

    let response_time = start.elapsed();

    // Get current time as UNIX timestamp
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

// Function to handle the monitoring of multiple websites concurrently
pub fn monitor_websites(config: &Config, websites: Vec<String>) -> Vec<WebsiteStatus> {
    let (tx, rx) = std::sync::mpsc::channel();
    let websites = Arc::new(Mutex::new(websites));

    let mut handles = vec![];

    for _ in 0..config.num_threads {
        let tx = tx.clone();
        let websites = Arc::clone(&websites);
        let timeout_duration = config.timeout_duration;

        let handle = thread::spawn(move || {
            loop {
                let url = {
                    let mut websites = websites.lock().unwrap();
                    if let Some(url) = websites.pop() {
                        Some(url)
                    } else {
                        None
                    }
                };

                if let Some(url) = url {
                    let status = check_website(url, timeout_duration);
                    tx.send(status).unwrap();
                } else {
                    break; // No websites left to process, exit the thread
                }
            }
        });

        handles.push(handle);
    }

    drop(tx); // No more sending of results after all threads are spawned

    let mut results = vec![];
    for status in rx {
        results.push(status);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results
}
