use std::{
    fs::File,
    io::{self, BufRead},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let common_words = vec![
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
    ];

    // let m = Mutex::new(vec![]);
    let m = Arc::new(Mutex::new(vec![]));

    let total_start = Instant::now();
    let mut children = vec![];

    for common_word in common_words.iter().cloned() {
        let m2 = Arc::clone(&m);
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
            let mut data = m2.lock().unwrap();
            data.push((common_word, count, duration));
        });

        children.push(child);
    }

    for child in children {
        child.join().unwrap();
    }

    let mut data = m.lock().unwrap();
    data.sort_by(|a, b| b.1.cmp(&a.1));

    for item in data.iter() {
        let (common_word, count, duration) = item;
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
