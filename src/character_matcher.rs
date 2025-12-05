use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    StartAnchor,
    EndAnchor,
    Literal(char),
    AnyChar,
    Digit,
    Word,
    Sequence(Vec<Token>, bool),
    Alternation(Vec<Token>),
    AlternationOption(Vec<Token>),
    Quantified { atom: Box<Token>, kind: Quantifier },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    OneOrMore,
    ZeroOrOne,
    None,
}

pub fn lexer(pattern: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut pattern = pattern.chars().peekable();
    while let Some(token) = pattern.next() {
        let token = match token {
            _ if token == '\\' => {
                if let Some(next_token) = pattern.next() {
                    match next_token {
                        _ if next_token == 'w' => Token::Word,
                        _ if next_token == 'd' => Token::Digit,
                        _ => Token::Word,
                    }
                } else {
                    Token::Literal('\\')
                }
            }
            _ if token == '(' => get_alternator_token(&mut pattern),
            _ if token == '[' => get_sequence_token(&mut pattern),
            _ if token == '^' => Token::StartAnchor,
            _ if token == '$' => Token::EndAnchor,
            _ if token == '.' => Token::AnyChar,
            _ if token == '+' => Token::Quantified {
                atom: Box::new(tokens.pop().unwrap()),
                kind: Quantifier::OneOrMore,
            },
            _ if token == '?' => Token::Quantified {
                atom: Box::new(tokens.pop().unwrap()),
                kind: Quantifier::ZeroOrOne,
            },
            _ => Token::Literal(token),
        };

        tokens.push(token);
    }

    tokens
}

pub fn get_sequence_token(pattern: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Token {
    let mut word: Vec<Token> = Vec::new();
    let mut negative = false;

    for char in pattern.by_ref() {
        match char {
            '^' => {
                negative = true;
            }
            ']' => return Token::Sequence(word, negative),
            _ => word.push(Token::Literal(char)),
        }
    }

    Token::Alternation(vec![])
}

pub fn get_alternator_token(pattern: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Token {
    let mut options: Vec<Token> = Vec::new();
    let mut word: Vec<Token> = Vec::new();

    for char in pattern.by_ref() {
        match char {
            '|' => {
                options.push(Token::AlternationOption(word.clone()));
                word.clear();
            }
            ')' => {
                options.push(Token::AlternationOption(word.clone()));
                return Token::Alternation(options);
            }
            _ => word.push(Token::Literal(char)),
        }
    }

    Token::Alternation(vec![])
}

pub fn grep(input: &str, pattern: &str) -> (bool, String) {
    let tokens = lexer(pattern);
    let chars: Vec<char> = input.chars().collect();
    let mut matches = String::new();

    for start_pos in 0..=chars.len() {
        let mut pos = start_pos;

        let mut t = tokens.clone();
        if let Some(Token::StartAnchor) = t.first() {
            if pos > 0 {
                continue;
            }
            t.remove(0);
        }

        if match_pattern(&chars, t, &mut pos, &mut matches) {
            return (true, matches);
        }
    }

    (false, String::new())
}

pub fn match_pattern(
    chars: &[char],
    tokens: Vec<Token>,
    pos: &mut usize,
    matches: &mut String,
) -> bool {
    let mut tokens: VecDeque<Token> = VecDeque::from(tokens);

    while let Some(token) = tokens.front() {
        let tokens_after: Vec<Token> = tokens.iter().skip(1).cloned().collect();
        let tokens_after_slice: &[Token] = &tokens_after;

        match token {
            Token::EndAnchor => {
                return pos == &chars.len();
            }
            Token::Quantified { atom, kind } => {
                match match_quantified(chars, atom, kind, pos, tokens_after_slice, matches) {
                    Ok(()) => {
                        return tokens_after_slice.is_empty() || *pos <= chars.len();
                    }
                    Err(_) => return false,
                }
            }
            Token::Sequence(seq_tokens, negative) => {
                match match_sequence(seq_tokens, chars, negative) {
                    Ok(()) => return true,
                    Err(_) => return false,
                }
            }
            Token::Alternation(alt_tokens) => {
                match match_alteration(alt_tokens, chars, pos, matches) {
                    Ok(()) => {
                        tokens.pop_front();
                    }
                    Err(_) => return false,
                }
            }
            _ => {
                let c = match chars.get(*pos) {
                    Some(c) => c,
                    None => return false,
                };

                if match_token(token, c) {
                    matches.push(c.to_owned());
                    *pos += 1;
                    tokens.pop_front();
                } else {
                    return false;
                }
            }
        }
    }

    true
}

fn match_sequence(tokens: &[Token], chars: &[char], negative: &bool) -> Result<(), ()> {
    for char in chars {
        let char_in_class = tokens.iter().any(|token| match_token(token, char));

        if *negative {
            if !char_in_class {
                return Ok(());
            }
        } else if char_in_class {
            return Ok(());
        }
    }

    Err(())
}

fn match_alteration(
    token: &Vec<Token>,
    chars: &[char],
    pos: &mut usize,
    matches: &mut String,
) -> Result<(), ()> {
    for option in token {
        match option {
            Token::AlternationOption(seq) => {
                let mut tmp_pos = *pos;

                match match_alternation_option(seq, chars, &mut tmp_pos, matches) {
                    Ok(()) => {
                        *pos = tmp_pos;
                        return Ok(());
                    }
                    Err(()) => {
                        continue;
                    }
                }
            }
            _ => {
                return Err(());
            }
        }
    }

    Err(())
}

fn match_alternation_option(
    tokens: &Vec<Token>,
    chars: &[char],
    pos: &mut usize,
    matches: &mut String,
) -> Result<(), ()> {
    let mut word = String::new();

    for token in tokens {
        if let Some(char) = chars.get(*pos) {
            if match_token(token, char) {
                word.push(char.to_owned());
                *pos += 1;
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
    }

    matches.push_str(&word);
    Ok(())
}

fn match_quantified(
    chars: &[char],
    atom: &Token,
    kind: &Quantifier,
    pos: &mut usize,
    tokens_after: &[Token],
    matches: &mut String,
) -> Result<(), ()> {
    match kind {
        Quantifier::OneOrMore => match_one_or_more(chars, atom, pos, tokens_after, matches),
        Quantifier::ZeroOrOne => match_zero_or_one(chars, atom, pos, tokens_after, matches),
        Quantifier::None => Err(()),
    }
}

fn match_one_or_more(
    chars: &[char],
    token: &Token,
    pos: &mut usize,
    tokens_after: &[Token],
    matches: &mut String,
) -> Result<(), ()> {
    let mut word = String::new();
    let start_pos = *pos;

    while let Some(&c) = chars.get(*pos) {
        if !match_token(token, &c) {
            break;
        }
        word.push(chars[*pos]);
        *pos += 1;
    }

    matches.push_str(&word);

    if *pos == start_pos {
        return Err(());
    }

    if tokens_after.is_empty() {
        return Ok(());
    }

    for trial_pos in (start_pos + 1..=*pos).rev() {
        let mut temp_pos = trial_pos;
        if match_pattern(chars, tokens_after.to_vec(), &mut temp_pos, matches) {
            *pos = temp_pos;
            return Ok(());
        }
    }
    Err(())
}

fn match_zero_or_one(
    chars: &[char],
    token: &Token,
    pos: &mut usize,
    tokens_after: &[Token],
    matches: &mut String,
) -> Result<(), ()> {
    if *pos >= chars.len() {
        return Ok(());
    }

    let temp_matches = chars[*pos];

    if match_token(token, &chars[*pos]) {
        let mut temp_pos = *pos + 1;
        if tokens_after.is_empty()
            || match_pattern(chars, tokens_after.to_vec(), &mut temp_pos, matches)
        {
            if tokens_after.is_empty() || matches!(tokens_after.first().unwrap(), Token::EndAnchor)
            {
                matches.push(temp_matches);
            } else {
                let prev = matches.pop().unwrap();
                matches.push(temp_matches);
                matches.push(prev);
            }

            *pos = temp_pos;
            return Ok(());
        }
    }

    let mut temp_pos = *pos;
    if tokens_after.is_empty()
        || match_pattern(chars, tokens_after.to_vec(), &mut temp_pos, matches)
    {
        *pos = temp_pos;
        return Ok(());
    }

    Err(())
}

fn match_token(token: &Token, c: &char) -> bool {
    match token {
        Token::Digit => digit(c),
        Token::Word => word_characters(c),
        Token::AnyChar => any_char(c),
        Token::Literal(s) => c == s,
        Token::EndAnchor => true,
        Token::StartAnchor => true,
        _ => false,
    }
}

fn any_char(char: &char) -> bool {
    char != &'\n'
}

fn digit(char: &char) -> bool {
    char.is_numeric()
}

fn word_characters(char: &char) -> bool {
    char.is_ascii_alphanumeric() || char == &'_'
}
