# grex
A small grep clone.

Given a string and a path, print only the lines that contain the given string.

Some optimizations:
- Uses `std::io::BufReader` to avoid reading the whole file into memory.
- Uses rust module system
- Adds error handling to parsing of command line arguments

## Installation
To install, run:
```
cargo install gr3x
```

## USAGE:
    gr3x <pattern> <path>
    
## SOURCE:
rust-cli.github.io/book/index.html
