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
    Group(Box<Token>),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    OneOrMore,
    ZeroOrOne,
    ZeroOrMore,
    NTimes(NMatch),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NMatch {
    pub n: i8,
    pub m: Option<i8>,
    pub atleast: bool,
}

pub fn lexer(pattern: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut groups: VecDeque<Token> = VecDeque::new();

    let mut pattern = pattern.chars().peekable();
    while let Some(token) = pattern.next() {
        let token = match_token(token, &mut pattern, &mut tokens, &mut groups);
        tokens.push(token);
    }

    println!("{:?}", tokens);

    tokens
}

fn match_token(
    token: char,
    pattern: &mut std::iter::Peekable<std::str::Chars<'_>>,
    tokens: &mut Vec<Token>,
    groups: &mut VecDeque<Token>,
) -> Token {
    match token {
        _ if token == '\\' => {
            if let Some(next_token) = pattern.next() {
                match next_token {
                    _ if next_token == 'w' => Token::Word,
                    _ if next_token == 'd' => Token::Digit,
                    _ if next_token.is_numeric() => {
                        Token::Group(Box::new(groups.pop_back().unwrap()))
                    }
                    _ => Token::Literal('\\'),
                }
            } else {
                Token::Literal('\\')
            }
        }
        _ if token == '(' => {
            let group = get_alternator_token(pattern, groups);
            groups.push_front(group.clone());
            group
        }
        _ if token == '[' => get_sequence_token(pattern),
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
        _ if token == '*' => Token::Quantified {
            atom: Box::new(tokens.pop().unwrap()),
            kind: Quantifier::ZeroOrMore,
        },
        _ if token == '{' => get_n_times_match_token(pattern, tokens),
        _ => Token::Literal(token),
    }
}

pub fn get_n_times_match_token(
    pattern: &mut std::iter::Peekable<std::str::Chars<'_>>,
    tokens: &mut Vec<Token>,
) -> Token {
    let mut n = 0;
    let mut m: Option<i8> = None;
    let mut atleast = false;

    while let Some(&ch) = pattern.peek() {
        match ch {
            '}' => {
                pattern.next();

                let options = NMatch { n, m, atleast };

                return Token::Quantified {
                    atom: Box::new(tokens.pop().unwrap()),
                    kind: Quantifier::NTimes(options),
                };
            }
            ',' => {
                atleast = true;
                pattern.next();
            }
            _ => {
                let number = ch.to_string().parse::<i8>().unwrap();
                if atleast {
                    atleast = false;
                    m = Some(number);
                } else {
                    n += number;
                }
                pattern.next();
            }
        }
    }

    Token::None
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

pub fn get_alternator_token(
    pattern: &mut std::iter::Peekable<std::str::Chars<'_>>,
    groups: &mut VecDeque<Token>,
) -> Token {
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
                let token = match_token(ch, pattern, &mut branch, groups);
                branch.push(token);
            }
        }
    }

    options.push(branch);
    Token::Alternation(options)
}
