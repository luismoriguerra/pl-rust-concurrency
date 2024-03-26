use std::{
    fs::File,
    io::{self, BufRead},
    thread,
    time::{Duration, Instant},
};

fn main() {
    let common_words = vec![
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
    ];

    let total_start = Instant::now();
    let mut children = vec![];

    for common_word in common_words.iter().cloned() {
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
            println!(
                "The word '{}' appears {} times in the file. Time elapsed: {:?}",
                common_word,
                count,
                duration.as_millis()
            );
        });

        children.push(child);
    }

    for child in children {
        child.join().unwrap();
    }

    let duration = total_start.elapsed();
    println!("Total time elapsed: {:?}", duration.as_millis());
}
