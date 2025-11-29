use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern == "\\d" {
        input_line.chars().any(|x| x.is_numeric())
    } else if pattern == "\\w" {
        input_line
            .chars()
            .any(|x| x.is_ascii_alphanumeric() || x == '_')
    } else {
        input_line.contains(pattern)
    }
}

fn main() {
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
