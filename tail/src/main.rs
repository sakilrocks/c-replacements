use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lines = 10;
    let mut file_index = 1;

    if args.len() >= 3 && args[1] == "-n" {
        lines = args[2].parse().unwrap_or(10);
        file_index = 3;
    }

    if args.len() <= file_index {
        eprintln!("Usage: tail [-n lines] <file>");
        std::process::exit(1);
    }

    let file = File::open(&args[file_index]).unwrap_or_else(|e| {
        eprintln!("tail: {}", e);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut buffer = VecDeque::with_capacity(lines);

    for line in reader.lines() {
        if buffer.len() == lines {
            buffer.pop_front();
        }
        buffer.push_back(line.unwrap());
    }

    for line in buffer {
        println!("{}", line);
    }
}