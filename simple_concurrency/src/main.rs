use std::thread;

fn simple_spawn_join() {

    let mut handles = vec![];

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {}", i); 
        });
        handles.push(handle); 
    }
    
    for handle in handles {
        handle.join().unwrap(); 
    }

    
    println!("All threads completed.");
}

fn main() {
    simple_spawn_join();
}
