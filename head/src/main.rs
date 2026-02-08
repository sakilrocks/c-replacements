use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lines = 10;
    let mut file_index = 1;

    if args.len() >= 3 && args[1] == "-n" {
        lines = args[2].parse().unwrap_or(10);
        file_index = 3;
    }

    if args.len() <= file_index {
        eprintln!("Usage: head [-n lines] <file>");
        std::process::exit(1);
    }

    let file = File::open(&args[file_index]).unwrap_or_else(|e| {
        eprintln!("head: {}", e);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);

    for line in reader.lines().take(lines) {
        println!("{}", line.unwrap());
    }
}