use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut pattern = pattern.to_string();
    let anchor = pattern.starts_with("^");
    if anchor {
        pattern.remove(0);
    }
    let end_anchor = pattern.ends_with("$");
    if end_anchor {
        pattern.remove(pattern.len() - 1);
    }

    let mut temp_pattern = pattern.to_string();
    println!("{}", end_anchor);

    for (i, c) in input_line.chars().enumerate() {
        println!("{}, {}, len{}", temp_pattern, i, input_line.len());
        if temp_pattern.starts_with("\\d") && digit(&c) {
            temp_pattern = temp_pattern.replacen("\\d", "", 1);
        } else if temp_pattern.starts_with("\\w") && word_characters(&c) {
            temp_pattern = temp_pattern.replacen("\\w", "", 1);
        } else if temp_pattern.starts_with(&c.to_string()) {
            temp_pattern = temp_pattern.replacen(&c.to_string(), "", 1);
        } else {
            if anchor {
                return false;
            }
            temp_pattern = pattern.to_string();
        }

        if temp_pattern.is_empty() && end_anchor && i == input_line.len() - 1 {
            return true;
        }

        if temp_pattern.is_empty() && !end_anchor {
            return true;
        }
    }

    false
}

fn digit(char: &char) -> bool {
    char.is_numeric()
}

fn word_characters(char: &char) -> bool {
    char.is_ascii_alphanumeric() || char == &'_'
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

    if pattern.starts_with("[^")
        && pattern.ends_with("]")
        && negative_character_group(&input_line, &pattern)
    {
        process::exit(0)
    }
    if pattern.starts_with("[")
        && !pattern.starts_with("[^")
        && pattern.ends_with("]")
        && positive_character_group(&input_line, &pattern)
    {
        process::exit(0)
    }

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
