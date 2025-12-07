use crate::grep::match_pattern;
use crate::lexer::Token;
use crate::utils::match_token;

pub fn match_one_or_more(
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

pub fn match_zero_or_more(
    chars: &[char],
    token: &Token,
    pos: usize,
    tokens_after: &[Token],
) -> Option<usize> {
    let mut end = pos;

    while let Some(next_pos) = match_pattern(chars, vec![token.to_owned()], end) {
        end = next_pos
    }

    if tokens_after.is_empty() || matches!(tokens_after.first().unwrap(), Token::EndAnchor) {
        return Some(end);
    }

    for candidate in (pos..=end).rev() {
        if let Some(next_pos) = match_pattern(chars, tokens_after.to_vec(), candidate) {
            return Some(next_pos);
        }
    }

    None
}

pub fn match_zero_or_one(
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
