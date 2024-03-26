use std::{
    fs::File,
    io::{self, BufRead},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() {
    let common_words = vec![
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
    ];

    let (tx, rx) = mpsc::channel();

    let total_start = Instant::now();
    let mut children = vec![];

    for common_word in common_words.iter().cloned() {
        let tx1 = tx.clone();

        let child = thread::spawn(move || {
            let start = Instant::now();
            let file = File::open("text.txt").expect("file not found");
            let reader = io::BufReader::new(file);

            let mut count = 0;

            for line in reader.lines() {
                let line = line.expect("line not found");
                let words = line.split_whitespace();

                for word in words {
                    if word.to_lowercase() == common_word {
                        count += 1;
                    }
                }
            }

            let duration = start.elapsed();
            tx1.send((common_word, count, duration));
        });

        children.push(child);
    }

    // this needs to be manually drop because the thread uses a copy of the tx
    // and the copy is the one that is being dropped
    drop(tx);

    for received in rx {
        let (common_word, count, duration) = received;
        println!(
            "The word '{}' appears {} times in the file. Time elapsed: {:?}",
            common_word,
            count,
            duration.as_millis()
        );
    }

    let duration = total_start.elapsed();
    println!("Total time elapsed: {:?}", duration.as_millis());
}
