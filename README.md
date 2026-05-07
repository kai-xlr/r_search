# r_search

A simple command-line search tool written in Rust.

## Usage

```bash
cargo run -- <query> <file_path>
```

Example:
```bash
cargo run -- hello file.txt
```

## Features

- Case-insensitive search
- Line numbers displayed for matching lines
- Error handling for missing arguments or unreadable files

## About

Searches for a query string within a text file and prints matching lines with line numbers. Exits with code 1 on error (missing arguments or file read failure).
