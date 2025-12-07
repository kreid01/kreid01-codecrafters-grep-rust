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
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    OneOrMore,
    ZeroOrOne,
    ZeroOrMore,
    NTimes(i8, bool),
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
            _ if token == '*' => Token::Quantified {
                atom: Box::new(tokens.pop().unwrap()),
                kind: Quantifier::ZeroOrMore,
            },
            _ if token == '{' => get_n_times_match_token(&mut pattern, &mut tokens),
            _ => Token::Literal(token),
        };

        tokens.push(token);
    }

    tokens
}

pub fn get_n_times_match_token(
    pattern: &mut std::iter::Peekable<std::str::Chars<'_>>,
    tokens: &mut Vec<Token>,
) -> Token {
    let mut n = 0;
    let mut atleast = false;

    while let Some(&ch) = pattern.peek() {
        match ch {
            '}' => {
                pattern.next();
                return Token::Quantified {
                    atom: Box::new(tokens.pop().unwrap()),
                    kind: Quantifier::NTimes(n, atleast),
                };
            }
            ',' => {
                atleast = true;
                pattern.next();
            }
            _ => {
                n += ch.to_string().parse::<i8>().unwrap();
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
