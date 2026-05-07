use std::env;
use std::fs;
use std::process;

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <query> <file_path>");
        process::exit(1);
    }

    (args[1].clone(), args[2].clone())
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Error: Could not read file '{}': {}", file_path, e);
        process::exit(1);
    })
}

fn search(query: &str, contents: &str) -> Vec<(usize, String)> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push((i + 1, line.to_string()));
        }
    }

    results
}

fn run() {
    let (query, file_path) = parse_args();
    let contents = read_file(&file_path);

    let results = search(&query, &contents);

    for (line_num, line) in results {
        println!("{}: {}", line_num, line);
    }
}

fn main() {
    run();
}
