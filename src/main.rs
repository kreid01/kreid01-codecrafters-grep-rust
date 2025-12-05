use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

mod character_matcher;

fn main() {
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut matched = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if character_matcher::grep(&line, &pattern) {
            matched = true;
            println!("{}", line);
        }
    }

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
