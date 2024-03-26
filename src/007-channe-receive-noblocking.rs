use std::{
    fs::File,
    io::{self, BufRead},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send("Hello, world!").unwrap();
    });

    // try to received the message immediately
    // non-blocking
    match rx.try_recv() {
        Ok(received) => println!("recieved: {}", received),
        Err(err) => println!("1 No message available yet: {:?}", err),
    }

    print!("main thread is working ...");
    thread::sleep(Duration::from_secs(4));

    // try to receive the message again
    match rx.try_recv() {
        Ok(received) => println!("Received in try 2: {}", received),
        Err(err) => println!("2 No message available yet: {:?}", err),
    }
}
