use crate::grep::match_pattern;
use crate::lexer::Token;
use crate::utils::match_token;

pub fn match_sequence(
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

pub fn match_alteration(
    alt_tokens: &[Vec<Token>],
    chars: &[char],
    start_pos: usize,
) -> Option<usize> {
    for branch in alt_tokens {
        if let Some(end_pos) = match_pattern(chars, branch.to_vec(), start_pos) {
            return Some(end_pos);
        }
    }
    None
}
