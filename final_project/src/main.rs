mod config;
mod website_monitor;

use crate::config::Config;
use crate::website_monitor::{monitor_websites, WebsiteStatus};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn read_urls_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut urls = Vec::new();

    // Read each line (URL) from the file
    for line in reader.lines() {
        if let Ok(url) = line {
            urls.push(url);
        }
    }

    Ok(urls)
}

fn main() {
    // Define configuration parameters
    let config = Config::new(4, 5, 3); // 4 threads, 5-second timeout, 3 retries

    // Specify the path to the file containing the URLs
    let file_path = "websites.txt";

    // Read URLs from the file
    let websites = match read_urls_from_file(file_path) {
        Ok(urls) => urls,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Monitor websites
    let website_statuses = monitor_websites(&config, websites);

    // Output the results
    for status in website_statuses {
        println!("{:?}", status);
    }
}
