use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rm <file>...");
        std::process::exit(1);
    }

    for file in &args[1..] {
        if let Err(e) = fs::remove_file(file) {
            eprintln!("rm: {}: {}", file, e);
        }
    }
    
}