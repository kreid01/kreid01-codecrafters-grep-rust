#[derive(Debug)]
pub enum Token {
    StartAnchor,
    EndAnchor,
    Literal(char),
    AnyChar,
    Digit,
    Word,
    Sequence(Vec<Token>),
    Quantified { atom: Box<Token>, kind: Quantifier },
}

#[derive(Debug)]
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
                    };
                }

                Token::Literal(token)
            }
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

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut tokens = lexer(pattern);

    let mut chars = input_line.chars().peekable();

    while let Some(c) = chars.next() {
        println!("char: {}, toks{:?}", c, tokens);
        if let Some(token) = tokens.first() {
            let matched = match_token(token, c, &Quantifier::None);

            if matched {
                tokens.remove(0);
            }
        }
    }

    tokens.is_empty()
}

fn match_token(token: &Token, c: char, q: &Quantifier) -> bool {
    let quantifier = match q {
        Quantifier::ZeroOrOne => true,
        Quantifier::OneOrMore => true,
        Quantifier::None => false,
    };

    if quantifier {
        return true;
    }

    match token {
        Token::Digit => digit(&c),
        Token::Word => word_characters(&c),
        Token::AnyChar => any_char(&c),
        Token::Literal(s) => &c == s,
        Token::Quantified { atom, kind } => match_token(atom, c, kind),
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
