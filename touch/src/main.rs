use std::env;
use std::fs::OpenOptions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: touch <file>...");
        std::process::exit(1);
    }

    for file in &args[1..] {
        if let Err(e) = OpenOptions::new().create(true).write(true).open(file) {
            eprintln!("touch: {}: {}", file, e);
        }
    }
    
}