use std::env;
use std::fs;
use std::io;

fn copy_file(src: &str, dst: &str) -> io::Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cp <source> <destination>");
        std::process::exit(1);
    }

    if let Err(e) = copy_file(&args[1], &args[2]) {
        eprintln!("cp: {}", e);
        std::process::exit(1);
    }
    
}