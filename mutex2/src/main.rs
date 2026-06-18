use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    let counter = Arc::new(Mutex::new(0)); //if mutex containes a rderence then still would not be safe, even if its ARC
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    } 

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}