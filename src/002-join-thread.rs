use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..11 {
            println!("Hello from a thread! {}", i);
            //blocking function
            thread::sleep(Duration::from_secs(2));
        }
    });

    println!("Hello from main!");
    for i in 1..11 {
        println!("Hello from main! {}", i);
        thread::sleep(Duration::from_secs(1));
    }
    //blocking function
    handler.join().unwrap();
    println!("Thread finished!");
}
