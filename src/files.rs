use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use crate::FileContents;

pub fn get_files_contents(arg_index: usize, recursive: bool) -> Vec<FileContents> {
    let mut files: Vec<FileContents> = Vec::new();
    let mut file_index = arg_index + 2;

    while let Some(file) = env::args().nth(file_index) {
        let path = Path::new(&file).to_path_buf();
        if path.is_dir()
            && let Some(mut contents) = get_dir_contents(&path, recursive)
        {
            files.append(contents.as_mut());
        } else if let Some(contents) = get_file_content(&path) {
            files.push(contents);
        }

        file_index += 1;
    }

    files
}

fn get_dir_contents(path: &Path, recursive: bool) -> Option<Vec<FileContents>> {
    let mut files: Vec<FileContents> = Vec::new();

    if path.is_dir() {
        for path in Path::read_dir(path).unwrap() {
            let path = path.unwrap().path().to_path_buf();
            if path.is_dir()
                && recursive
                && let Some(mut contents) = get_dir_contents(&path, recursive)
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
