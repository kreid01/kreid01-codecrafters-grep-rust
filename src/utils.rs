use crate::lexer::Token;

pub fn match_token(token: &Token, c: &char) -> bool {
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
