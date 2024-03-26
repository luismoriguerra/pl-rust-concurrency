use std::thread;

fn main() {
    thread::spawn(|| {
        // this wont execute if the main thread exits before this thread
        // this is race condition
        println!("Hello from a thread!");
    });

    println!("Hello from main!");
}
