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
    Alternation(Vec<Vec<Token>>),
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
    let mut options: Vec<Vec<Token>> = Vec::new();
    let mut branch: Vec<Token> = Vec::new();

    while let Some(&ch) = pattern.peek() {
        match ch {
            '|' => {
                pattern.next();
                options.push(branch.clone());
                branch.clear();
            }
            ')' => {
                pattern.next();
                options.push(branch.clone());
                return Token::Alternation(options);
            }
            _ => {
                pattern.next();
                branch.push(Token::Literal(ch));
            }
        }
    }

    options.push(branch);
    Token::Alternation(options)
}

pub fn grep(input: &str, pattern: &str) -> Vec<String> {
    let tokens = lexer(pattern);
    let chars: Vec<char> = input.chars().collect();
    let mut results = Vec::new();
    let mut start_pos = 0;

    while start_pos <= chars.len() {
        let mut tokens_clone = tokens.clone();

        if let Some(Token::StartAnchor) = tokens_clone.first() {
            if start_pos != 0 {
                break;
            }
            tokens_clone.remove(0);
        }

        if let Some(end_pos) = match_pattern(&chars, tokens_clone, &mut start_pos) {
            println!("end {}", end_pos);
            results.push(chars[start_pos..end_pos].iter().collect());
            start_pos = end_pos;
        } else {
            start_pos += 1;
        }
    }

    results
}

pub fn match_pattern(chars: &[char], tokens: Vec<Token>, pos: &mut usize) -> Option<usize> {
    let mut tokens: VecDeque<Token> = VecDeque::from(tokens);

    while let Some(token) = tokens.front() {
        let tokens_after: Vec<Token> = tokens.iter().skip(1).cloned().collect();
        let tokens_after_slice: &[Token] = &tokens_after;

        match token {
            Token::EndAnchor => return Some(*pos),
            Token::Quantified { atom, kind } => {
                return match_quantified(chars, atom, kind, pos, tokens_after_slice);
            }
            Token::Sequence(seq_tokens, _negative) => {
                return match_pattern(chars, seq_tokens.to_vec(), pos);
            }
            Token::Alternation(alt_tokens) => {
                return match_alteration(alt_tokens, chars, pos);
            }
            _ => {
                let c = chars.get(*pos)?;
                if match_token(token, c) {
                    *pos += 1;
                } else {
                    return None;
                }
            }
        }

        tokens.pop_front();
    }

    Some(*pos)
}

fn match_alteration(
    alt_tokens: &[Vec<Token>],
    chars: &[char],
    start_pos: &mut usize,
) -> Option<usize> {
    for branch in alt_tokens {
        if let Some(end_pos) = match_pattern(chars, branch.to_vec(), start_pos) {
            return Some(end_pos);
        }
    }
    None
}

fn match_quantified(
    chars: &[char],
    atom: &Token,
    kind: &Quantifier,
    pos: &mut usize,
    tokens_after: &[Token],
) -> Option<usize> {
    match kind {
        Quantifier::OneOrMore => match_one_or_more(chars, atom, pos, tokens_after),
        Quantifier::ZeroOrOne => match_zero_or_one(chars, atom, pos, tokens_after),
        Quantifier::None => None,
    }
}

fn match_one_or_more(
    chars: &[char],
    token: &Token,
    pos: &mut usize,
    tokens_after: &[Token],
) -> Option<usize> {
    let start_pos = *pos;

    while let Some(&c) = chars.get(*pos) {
        if !match_token(token, &c) {
            break;
        }
        *pos += 1;
    }

    if *pos == start_pos {
        return None;
    }

    if tokens_after.is_empty() {
        return Some(*pos);
    }

    for trial_pos in (start_pos + 1..=*pos).rev() {
        let mut temp_pos = trial_pos;
        if let Some(pos) = match_pattern(chars, tokens_after.to_vec(), &mut temp_pos) {
            return Some(pos);
        }
    }

    None
}

fn match_zero_or_one(
    chars: &[char],
    token: &Token,
    pos: &mut usize,
    tokens_after: &[Token],
) -> Option<usize> {
    if *pos >= chars.len() {
        return Some(*pos);
    }

    if match_token(token, &chars[*pos]) {
        let mut temp_pos = *pos + 1;
        if tokens_after.is_empty() || matches!(tokens_after.first().unwrap(), Token::EndAnchor) {
            return Some(*pos);
        }

        if let Some(pos) = match_pattern(chars, tokens_after.to_vec(), &mut temp_pos) {
            return Some(pos);
        }
    }

    let mut temp_pos = *pos;
    if tokens_after.is_empty() {
        return Some(*pos);
    }

    if let Some(pos) = match_pattern(chars, tokens_after.to_vec(), &mut temp_pos) {
        return Some(pos);
    }

    None
}

fn match_token(token: &Token, c: &char) -> bool {
    match token {
        Token::Digit => digit(c),
        Token::Word => word_characters(c),
        Token::AnyChar => any_char(c),
        Token::Literal(s) => c == s,
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
