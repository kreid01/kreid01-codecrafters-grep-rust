pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
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
    let mut greed = false;
    let mut greed_symbol = "\\n".to_owned();

    let mut chars = input_line.chars().peekable();

    while let Some(c) = chars.next() {
        println!(
            "char: {}, pattern:{}, greed:{}",
            c, temp_pattern, &greed_symbol
        );

        if let Some(&next) = chars.peek()
            && greed
            && match_symbol(&greed_symbol, next)
        {
            continue;
        } else if greed && match_symbol(&greed_symbol, c) {
            temp_pattern = temp_pattern.replace(c, "");
            continue;
        } else {
            greed = false;
        }

        let mut temp_greed_symbol: &str = "\\n";
        let temp_c = &c.to_string();

        if temp_pattern.starts_with("\\d") && digit(&c) {
            temp_pattern = temp_pattern.replacen("\\d", "", 1);
            temp_greed_symbol = "\\d";
        } else if temp_pattern.starts_with("\\w") && word_characters(&c) {
            temp_pattern = temp_pattern.replacen("\\w", "", 1);
            temp_greed_symbol = "\\w";
        } else if temp_pattern.starts_with(&c.to_string()) {
            temp_pattern = temp_pattern.replacen(&c.to_string(), "", 1);
            temp_greed_symbol = temp_c
        } else {
            if anchor {
                return false;
            }
            temp_pattern = pattern.to_string();
        }

        if temp_pattern.starts_with("+") {
            greed = true;
            greed_symbol = temp_greed_symbol.to_string();
            temp_pattern = temp_pattern.replacen("+", "", 1);
        }

        if temp_pattern.is_empty() && end_anchor && chars.next().is_none() {
            return true;
        }

        if temp_pattern.is_empty() && !end_anchor {
            return true;
        }
    }

    false
}

fn match_symbol(pattern: &str, c: char) -> bool {
    match pattern {
        _ if pattern == "\\d" => digit(&c),
        _ if pattern == "\\w" => word_characters(&c),
        _ => c.to_string() == pattern,
    }
}

fn digit(char: &char) -> bool {
    char.is_numeric()
}

fn word_characters(char: &char) -> bool {
    char.is_ascii_alphanumeric() || char == &'_'
}
