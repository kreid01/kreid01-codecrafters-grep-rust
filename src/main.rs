use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

mod files;
mod grep;
mod lexer;
mod quantifiers;
mod sequence;
mod utils;

#[derive(Debug)]
struct FileContents {
    file_name: String,
    contents: Vec<String>,
}

fn main() {
    eprintln!("Logs from your program will appear here!");
    let mut e_index = 1;

    let mut show_matches = false;
    let mut recursive = false;

    if env::args().nth(1).unwrap() == "-o" {
        show_matches = true;
        e_index = 2;
    }

    if env::args().nth(1).unwrap() == "-r" {
        recursive = true;
        e_index = 2;
    }

    if env::args().nth(e_index).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let pattern = env::args().nth(e_index + 1).unwrap();
    let file_contents = files::get_files_contents(e_index, recursive);

    let matched = match file_contents.is_empty() {
        true => reader
            .lines()
            .map(|x| grep_line(&x.unwrap(), &pattern, show_matches))
            .fold(false, |acc, val| acc || val),
        false => file_contents
            .iter()
            .map(|x| grep_file(x, &pattern, file_contents.len() > 1))
            .fold(false, |acc, val| acc || val),
    };

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn grep_file(file: &FileContents, pattern: &str, multi: bool) -> bool {
    let mut matched = false;

    for line in &file.contents {
        let matches = grep::grep(line, pattern);
        if !matches.is_empty() {
            matched = true;
            for _ in matches {
                if multi {
                    let output = format!("{}:{}", file.file_name, line);
                    println!("{}", output);
                } else {
                    println!("{}", line);
                }
            }
        }
    }

    matched
}

fn grep_line(line: &str, pattern: &str, show_matches: bool) -> bool {
    let mut matched = false;
    let matches = grep::grep(line, pattern);
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
