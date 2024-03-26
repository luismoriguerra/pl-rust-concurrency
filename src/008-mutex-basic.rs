use std::{
    fs::File,
    io::{self, BufRead},
    sync::{mpsc, Mutex},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let m = Mutex::new(10);

    {
        let mut numb = m.lock().unwrap();
        *numb = 20;
    } // Mutex is unlocked here automatically

    println!("m = {:?}", m); // Mutex { data: 20 }

    let num = m.lock().unwrap();
    println!("num = {}", *num); // num = 20

}
