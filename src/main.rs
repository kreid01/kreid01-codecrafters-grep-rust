use std::env;
use std::fs::File;
use std::fs::read_to_string;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process;

mod character_matcher;

fn main() {
    eprintln!("Logs from your program will appear here!");
    let mut e_index = 1;
    let mut show_matches = false;

    if env::args().nth(1).unwrap() == "-o" {
        show_matches = true;
        e_index = 2;
    }

    if env::args().nth(e_index).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let pattern = env::args().nth(e_index + 1).unwrap();
    let file_lines = get_file_lines(e_index);

    let matched = match file_lines.is_empty() {
        true => reader
            .lines()
            .any(|x| grep_line(&x.unwrap(), &pattern, show_matches)),
        false => file_lines
            .iter()
            .any(|x| grep_line(x, &pattern, show_matches)),
    };

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn get_file_lines(arg_index: usize) -> Vec<String> {
    let mut file_lines: Vec<String> = Vec::new();
    if let Some(file) = env::args().nth(arg_index + 2) {
        let path = Path::new(&file).to_path_buf();
        if path.is_file()
            && let Ok(lines) = read_to_string(path)
        {
            file_lines = lines.lines().map(|x| x.to_string()).collect();
        }
    }

    file_lines
}

fn grep_line(line: &str, pattern: &str, show_matches: bool) -> bool {
    let mut matched = false;
    let matches = character_matcher::grep(line, pattern);
    if !matches.is_empty() {
        matched = true;
        if show_matches {
            println!("{}", matches.join("\n"));
        } else {
            println!("{}", line);
        }
    }

    matched
}
