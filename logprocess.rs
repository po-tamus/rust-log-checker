use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::time::Instant;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <filename> <search_string> <num_threads>", args[0]);
        return;
    }

    let filename = &args[1];
    let search_string = &args[2];
    let num_threads: usize = args[3].parse().expect("Invalid number of threads");

    let start = Instant::now();

    let path = Path::new(&filename);
    let file = File::open(&path).expect("Cannot open file");
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();

    let chunk_size = (lines.len() + num_threads - 1) / num_threads;
    let mut handles = Vec::new();

    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_owned(); // clone chunk
        let search_string = search_string.clone(); // clone for thread

        let handle = thread::spawn(move || {
            for line in chunk {
                if line.contains(&search_string) {
                    println!("{}", line);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("{:?} processed in {:?} nanoseconds", args[1], duration);
}
