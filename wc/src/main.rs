use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

fn count_file(path: &str) -> io::Result<Counts> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut counts = Counts::default();

    for line in reader.lines() {
        let line = line?;
        counts.lines += 1;
        counts.words += line.split_whitespace().count();
        counts.bytes += line.len() + 1;
    }

    Ok(counts)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: wc <file>");
        std::process::exit(1);
    }

    match count_file(&args[1]) {
        Ok(c) => println!("{} {} {} {}", c.lines, c.words, c.bytes, args[1]),
        Err(e) => {
            eprintln!("wc: {}", e);
            std::process::exit(1);
        }
    }
    
}
