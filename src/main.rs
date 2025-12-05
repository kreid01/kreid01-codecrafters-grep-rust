use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
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

    let pattern = env::args().nth(e_index + 1).unwrap();
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut matched = false;

    for line in reader.lines() {
        let line = line.unwrap();
        let (m, matches) = character_matcher::grep(&line, &pattern);
        if m {
            matched = true;
            if show_matches {
                println!("{}", matches);
            } else {
                println!("{}", line);
            }
        }
    }

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
