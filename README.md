# c-replacements

a collection of classic Unix style utilities rewritten in Rust as safe replacements for traditional C programs.   
Focused on memory safety, clarity, and correctness, while keeping behavior close to familiar core utils.

---

## Features

- Pure safe Rust (no unsafe blocks)
- Stream based file and IO handling
- Clear and consistent error messages
- Each tool implemented as an independent binary
- Easy to extend with new commands

---

## Project structure

```

c-replacements/
├── Cargo.toml         # Workspace definition
├── wc/                # Word, line, byte counter
├── cat/               # Print file contents
├── cp/                # Copy files
├── head/              # Print first N lines
├── tail/              # Print last N lines
├── mkdir/             # Create directories
├── rm/                # Remove files (non-recursive)
└── touch/             # Create empty files / update timestamps

```
*each folder is a standalone Cargo binary crate inside a single workspace.*

---

## How each tool works

```wc```
: Reads a file line by line using buffered IO and counts lines, words, and bytes without loading the entire file into memory.

```cat```
: Opens a file and writes its contents directly to stdout. Designed as a simple, safe replacement for the C version.

```cp```
: Uses Rust’s filesystem API to copy files safely, handling errors explicitly instead of relying on implicit behavior.

```head```
: Reads and prints only the first N lines of a file, stopping early to avoid unnecessary IO.

```tail```
: Maintains a fixed-size buffer to keep only the last N lines, allowing efficient handling of large files.

```mkdir```
: Creates one or more directories using Rust’s standard filesystem interface.

```rm```
: Removes files explicitly and safely (no recursive deletion) to avoid accidental data loss.

```touch```
: Creates empty files if they don’t exist or updates timestamps if they do.

---

## Build

From the project root:

```cargo build```

This builds all tools in the workspace.

---

## Run and usage

Run a specific tool using the -p flag:
  
``` 
cargo run -p wc -- file.txt
```    
``` 
cargo run -p cat -- file.txt
```     
``` 
cargo run -p cp -- source.txt destination.txt
```    
```
cargo run -p head -- -n 5 file.txt
```     
``` 
cargo run -p tail -- -n 10 file.txt
```   
```
cargo run -p mkdir -- example_dir 
```   
``` 
cargo run -p rm -- example.txt
```   
```
cargo run -p touch -- a.txt b.txt 
```   
