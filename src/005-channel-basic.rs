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
        tx.send("Hello, world!").unwrap();
    });

    // let received = rx.recv().unwrap();
    // or
    // let received = rx.recv().expect("Failed to receive a message");
    // println!("Got: {}", received);

    // best practice
    match rx.recv() {
        Ok(received) => println!("Got: {}", received),
        Err(err) => println!("Got an error: {:?}", err),
    }
}
