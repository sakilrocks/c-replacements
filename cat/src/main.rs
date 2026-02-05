use std::env;
use std::fs::File;
use std::io::{self, Read};

fn cat_file(path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    print!("{}", buffer);
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cat <file>");
        std::process::exit(1);
    }

    if let Err(e) = cat_file(&args[1]) {
        eprintln!("cat: {}", e);
        std::process::exit(1);
    };
    
}
