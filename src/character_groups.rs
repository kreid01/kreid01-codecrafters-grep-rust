use std::process;

pub fn check_character_groups(input_line: &str, pattern: &str) {
    if pattern.starts_with("[^")
        && pattern.ends_with("]")
        && negative_character_group(input_line, pattern)
    {
        process::exit(0)
    }
    if pattern.starts_with("[")
        && !pattern.starts_with("[^")
        && pattern.ends_with("]")
        && positive_character_group(input_line, pattern)
    {
        process::exit(0)
    }
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
