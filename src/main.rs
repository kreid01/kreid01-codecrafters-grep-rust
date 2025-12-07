use std::env;
use std::fs::read_to_string;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process;

mod character_matcher;

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
    let file_contents = get_files_contents(e_index);

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

#[derive(Debug)]
struct FileContents {
    file_name: String,
    contents: Vec<String>,
}

fn get_files_contents(arg_index: usize) -> Vec<FileContents> {
    let mut files: Vec<FileContents> = Vec::new();
    let mut file_index = arg_index + 2;

    while let Some(file) = env::args().nth(file_index) {
        let path = Path::new(&file).to_path_buf();
        if path.is_dir()
            && let Some(mut contents) = get_dir_contents(&path)
        {
            files.append(contents.as_mut());
        } else if let Some(contents) = get_file_content(&path) {
            files.push(contents);
        }

        file_index += 1;
    }

    files
}

fn get_dir_contents(path: &PathBuf) -> Option<Vec<FileContents>> {
    let mut files: Vec<FileContents> = Vec::new();

    if path.is_dir() {
        for path in Path::read_dir(path.as_path()).unwrap() {
            let path = path.unwrap().path().to_path_buf();
            if path.is_dir()
                && let Some(mut contents) = get_dir_contents(&path)
            {
                files.append(contents.as_mut());
            } else if let Some(contents) = get_file_content(&path) {
                files.push(contents);
            }
        }
    }

    Some(files)
}

fn get_file_content(path: &PathBuf) -> Option<FileContents> {
    if path.is_file()
        && let Ok(lines) = read_to_string(path)
    {
        let file_name = path.to_string_lossy().to_string();
        let contents = lines.lines().map(|x| x.to_string()).collect();
        let file_contents = FileContents {
            file_name,
            contents,
        };
        return Some(file_contents);
    }

    None
}

fn grep_file(file: &FileContents, pattern: &str, multi: bool) -> bool {
    let mut matched = false;

    for line in &file.contents {
        let matches = character_matcher::grep(line, pattern);
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
