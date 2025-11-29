use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern == "\\d" {
        digit(input_line)
    } else if pattern == "\\w" {
        word_characters(input_line)
    } else if pattern.starts_with("[^") && pattern.ends_with("]") {
        negative_character_group(input_line, pattern)
    } else if pattern.starts_with("[") && pattern.ends_with("]") {
        positive_character_group(input_line, pattern)
    } else {
        input_line.contains(pattern)
    }
}

fn digit(input_line: &str) -> bool {
    input_line.chars().any(|x| x.is_numeric())
}

fn word_characters(input_line: &str) -> bool {
    input_line
        .chars()
        .any(|x| x.is_ascii_alphanumeric() || x == '_')
}

fn positive_character_group(input_line: &str, pattern: &str) -> bool {
    let pattern: Vec<String> = pattern
        .replace("[", "")
        .replace("]", "")
        .split("")
        .map(|x| x.to_string())
        .collect();
    input_line.chars().any(|x| pattern.contains(&x.to_string()))
}

fn negative_character_group(input_line: &str, pattern: &str) -> bool {
    let pattern: Vec<String> = pattern
        .replace("[^", "")
        .replace("]", "")
        .split("")
        .map(|x| x.to_string())
        .collect();
    input_line
        .chars()
        .any(|x| !pattern.contains(&x.to_string()))
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
