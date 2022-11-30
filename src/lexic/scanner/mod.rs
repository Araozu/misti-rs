use super::{token::{TokenType, self}, utils, LexResult};

mod number;
mod operator;

/// Attempts to scan a number. Returns None to be able to chain other scanner
pub fn number(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    if utils::is_digit(c) {
        match number::scan(chars, start_pos) {
            Ok((token, next_pos)) => {
                Some(LexResult::Some(token, next_pos))
            },
            Err(reason) => {
                Some(LexResult::Err(reason))
            },
        }
    }
    else {
        None
    }
}


/// Attempts to scan an operator. Returns None to be able to chain other scanner
pub fn operator(c: char, chars: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    if utils::is_operator(c) {
        Some(operator::scan(chars, start_pos))
    }
    else {
        None
    }
}


/// Attempts to scan a grouping sign. Returns None to be able to chain other scanner
pub fn grouping_sign(c: char, _: &Vec<char>, start_pos: usize) -> Option<LexResult> {
    let token_type = match c {
        '(' => TokenType::LeftParen,
        ')' => TokenType::RightParen,
        '[' => TokenType::LeftBracket,
        ']' => TokenType::RightBracket,
        '{' => TokenType::LeftBrace,
        '}' => TokenType::RightBrace,
        _ => return None,
    };

    let token = token::new_grouping_sign(
        c.to_string(), 
        start_pos as i32, 
        token_type,
    );
    Some(LexResult::Some(token, start_pos + 1))
}
