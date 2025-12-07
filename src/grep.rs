use crate::lexer::{Quantifier, Token, lexer};
use crate::quantifiers::{match_one_or_more, match_zero_or_more, match_zero_or_one};
use crate::sequence::{match_alteration, match_sequence};
use crate::utils::match_token;
use std::collections::VecDeque;

pub fn grep(input: &str, pattern: &str) -> Vec<String> {
    let tokens = lexer(pattern);
    let chars: Vec<char> = input.chars().collect();
    let mut results = Vec::new();
    let mut start_pos = 0;

    while start_pos < chars.len() {
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
                Quantifier::ZeroOrMore => {
                    return match_zero_or_more(chars, atom, temp_pos, tokens_after_slice);
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
