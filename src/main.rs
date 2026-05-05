use std::env;
use std::fs;
use std::process;

fn search(query: &str, contents: &str) -> bool {
    let query = query.to_lowercase();
    let mut found = false;

    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            println!("{}: {}", i + 1, line);
            found = true;
        }
    }

    found
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <query> <file_path>");
        process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    let contents = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: Could not read file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    search(query, &contents);
    // success → implicit exit(0)
}
