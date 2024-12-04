use std::sync::{Arc, Mutex};
use std::thread;

fn share_data_between_threads_with_arc() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    println!("Spawned 5 threads!");

    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = *counter.lock().unwrap();
    println!("Final value of the counter: {}", final_value);
}

fn main() {
    share_data_between_threads_with_arc();
}
