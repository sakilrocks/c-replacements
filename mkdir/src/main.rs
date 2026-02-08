use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: mkdir <dir>...");
        std::process::exit(1);
    }

    for dir in &args[1..] {
        if let Err(e) = fs::create_dir(dir) {
            eprintln!("mkdir: {}: {}", dir, e);
        }
    }
}