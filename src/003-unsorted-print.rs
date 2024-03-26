use std::{thread, time::Duration};

fn main() {
    let mut handles = vec![];

    for i in 0..11 {
        let handle = thread::spawn(move || {
            // this will print unsorted values
            println!("Hello from a thread! {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
