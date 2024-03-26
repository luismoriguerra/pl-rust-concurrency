use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..11 {
            // this never finish to count to 10
            println!("Hello from a thread! {}", i);
            thread::sleep(Duration::from_secs(2));
        }
    });

    println!("Hello from main!");
    for i in 1..11 {
        println!("Hello from main! {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}
