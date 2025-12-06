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
                        _ => Token::Literal('\\'),
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

        if let Some(end_pos) = match_pattern(&chars, tokens_clone, start_pos) {
            results.push(chars[start_pos..end_pos].iter().collect());
            start_pos = end_pos;
        } else {
            start_pos += 1;
        }
    }

    results
}

pub fn match_pattern(chars: &[char], tokens: Vec<Token>, pos: usize) -> Option<usize> {
    let mut tokens: VecDeque<Token> = VecDeque::from(tokens);
    let mut temp_pos = pos;

    while let Some(token) = tokens.front() {
        let tokens_after: Vec<Token> = tokens.iter().skip(1).cloned().collect();
        let tokens_after_slice: &[Token] = &tokens_after;

        match token {
            Token::EndAnchor => {
                if temp_pos == chars.len() {
                    return Some(temp_pos);
                } else {
                    return None;
                }
            }
            Token::Quantified { atom, kind } => match kind {
                Quantifier::OneOrMore => {
                    return match_one_or_more(chars, atom, temp_pos, tokens_after_slice);
                }
                Quantifier::ZeroOrOne => {
                    if tokens_after_slice.is_empty() {
                        return match_zero_or_one(chars, atom, temp_pos, tokens_after_slice);
                    }
                    if let Some(end_pos) =
                        match_zero_or_one(chars, atom, temp_pos, tokens_after_slice)
                    {
                        temp_pos = end_pos
                    }
                }
                Quantifier::None => {
                    return None;
                }
            },
            Token::Sequence(seq_tokens, negative) => {
                if tokens_after_slice.is_empty() {
                    return match_sequence(seq_tokens.to_vec(), chars, temp_pos, *negative);
                }
                if let Some(end_pos) =
                    match_sequence(seq_tokens.to_vec(), chars, temp_pos, *negative)
                {
                    temp_pos = end_pos
                }
            }
            Token::Alternation(alt_tokens) => {
                if tokens_after_slice.is_empty() {
                    return match_alteration(alt_tokens, chars, temp_pos);
                }
                if let Some(end_pos) = match_alteration(alt_tokens, chars, temp_pos) {
                    temp_pos = end_pos
                }
            }
            _ => {
                let c = chars.get(temp_pos)?;
                if match_token(token, c) {
                    temp_pos += 1;
                } else {
                    return None;
                }
            }
        }

        tokens.pop_front();
    }

    Some(temp_pos)
}

fn match_sequence(
    seq_tokens: Vec<Token>,
    chars: &[char],
    start_pos: usize,
    negative: bool,
) -> Option<usize> {
    let c = chars.get(start_pos)?;

    if negative {
        for token in seq_tokens {
            if match_token(&token, c) {
                return None;
            }
        }
        Some(start_pos + 1)
    } else {
        for token in seq_tokens {
            if match_token(&token, c) {
                return Some(start_pos + 1);
            }
        }
        None
    }
}

fn match_alteration(alt_tokens: &[Vec<Token>], chars: &[char], start_pos: usize) -> Option<usize> {
    for branch in alt_tokens {
        if let Some(end_pos) = match_pattern(chars, branch.to_vec(), start_pos) {
            return Some(end_pos);
        }
    }
    None
}

fn match_one_or_more(
    chars: &[char],
    token: &Token,
    pos: usize,
    tokens_after: &[Token],
) -> Option<usize> {
    let mut end = pos;

    while let Some(&c) = chars.get(end) {
        if !match_token(token, &c) {
            break;
        }
        end += 1;
    }

    if end == pos {
        return None;
    }

    if end != pos && tokens_after.is_empty() {
        return Some(end);
    }

    for candidate in (pos..=end).rev() {
        if let Some(next_pos) = match_pattern(chars, tokens_after.to_vec(), candidate) {
            return Some(next_pos);
        }
    }

    None
}

fn match_zero_or_one(
    chars: &[char],
    token: &Token,
    pos: usize,
    tokens_after: &[Token],
) -> Option<usize> {
    if pos >= chars.len() {
        return Some(pos);
    }

    if match_token(token, &chars[pos]) {
        return Some(pos + 1);
    }

    if tokens_after.is_empty() || matches!(tokens_after.first().unwrap(), Token::EndAnchor) {
        return Some(pos);
    }

    if let Some(pos) = match_pattern(chars, tokens_after.to_vec(), pos + 1) {
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
