use std::env; // command line args
use std::path::Path; // file paths
use std::fs::File; // opening files 
use std::io::{self, BufRead}; // buffered read 
use std::vec::Vec; // dynamic array, single threaded???
use std::time::Instant; // timing 
use std::thread; // threading

fn main() {
    // iterate over the cmd line args with the collect function
    let args: Vec<String> = env::args().collect();

    // must be 4 comd line argumetns
    if args.len() != 4 {
        eprintln!("Usage: {} <filename> <search_string> <num_threads>", args[0]);
        return;
    }

    // get each value from the args vector 
    let filename = &args[1];
    let search_string = &args[2];
    let num_threads: usize = args[3].parse().expect("invalid count");

    // positive num of threads 
    if (num_threads <= 0) {
        eprintln!("Invalid thread count.");
        return;
    }

    // start time
    let start = Instant::now();


    let path = Path::new(&filename);
    let file = File::open(&path).expect("file doesn't exists");
    let reader = io::BufReader::new(file);

    // map the lines from the text
    /*
        map the lines from the text
        if any are 
    */
    let lines: Vec<String> = reader.lines().map(|line| line.expect("error")).collect();

    // how many lines each thread handles (rounded up)
    let lines_per_thread = (lines.len() + num_threads - 1) / num_threads;
    // store the thread handles 
    let mut handles = Vec::new();

    // splits lines vec into groups of size lines_per_thread
    for group_of_lines in lines.chunks(lines_per_thread) {
        // copies each chunk into a new vec
        let group_of_lines = group_of_lines.to_owned(); 
        // copied into the thread's scope 
        let search_string = search_string.clone(); 

        // create new threads
        let handle = thread::spawn(move || {
            for line in group_of_lines {
                if line.contains(&search_string) {
                    println!("{}", line);
                }
            }
        });

        // add the thread handle to handles vec
        handles.push(handle);
    }

    // wait for each to finish 
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed().as_nanos();
    println!("{:?} processed in {:?} nanoseconds", args[1], duration);
}
