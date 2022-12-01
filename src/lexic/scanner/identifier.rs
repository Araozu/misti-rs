use crate::lexic::{token, utils, LexResult};

pub fn scan(start_char: char, chars: &Vec<char>, start_pos: usize) -> LexResult {
    scan_impl(chars, start_pos + 1, format!("{}", start_char))
}

pub fn scan_impl(chars: &Vec<char>, start_pos: usize, current: String) -> LexResult {
    match chars.get(start_pos) {
        Some(c) if utils::is_identifier_char(*c) => {
            scan_impl(chars, start_pos + 1, utils::str_append(current, *c))
        },
        _ => {
            LexResult::Some(token::new_identifier(current, start_pos as i32), start_pos)
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::lexic::token::TokenType;

    use super::*;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    // Should scan a lenght 1 identifier
    #[test]
    fn test_1() {
        let input = str_to_vec("_");
        let start_pos = 0;
        match scan(*input.get(0).unwrap(), &input, start_pos) {
            LexResult::Some(token, next) => {
                assert_eq!(1, next);
                assert_eq!(TokenType::Identifier, token.token_type);
                assert_eq!("_", token.value);
            },
            _ => panic!()
        }

        let input = str_to_vec("i");
        let start_pos = 0;
        match scan(*input.get(0).unwrap(), &input, start_pos) {
            LexResult::Some(token, next) => {
                assert_eq!(1, next);
                assert_eq!(TokenType::Identifier, token.token_type);
                assert_eq!("i", token.value);
            },
            _ => panic!()
        }
    }

    // Should scan a lenght 2 identifier
    #[test]
    fn test_2() {
        let operators = vec![
            "_a",
            "_z",
            "_A",
            "_Z",
            "__",
            "_0",
            "_9",
            "aa",
            "az",
            "aA",
            "aZ",
            "a_",
            "a0",
            "a9",
            "za",
            "zz",
            "zA",
            "zZ",
            "z_",
            "z0",
            "z9",
        ];

        for op in operators {
            let input = str_to_vec(op);
            let start_pos = 0;
            match scan(*input.get(0).unwrap(), &input, start_pos) {
                LexResult::Some(token, next) => {
                    assert_eq!(2, next);
                    assert_eq!(TokenType::Identifier, token.token_type);
                    assert_eq!(op, token.value);
                },
                _ => panic!()
            }
        }
    }


    // Should scan long identifiers
    #[test]
    fn test_3() {
        let operators = vec![
            "_validIdentifier",
            "iterationCount",
            "buffer",
            "aVeryLongIdentifier2WithSome5Numbers67InBetween1",
        ];

        for op in operators {
            let input = str_to_vec(op);
            let start_pos = 0;
            match scan(*input.get(0).unwrap(), &input, start_pos) {
                LexResult::Some(token, next) => {
                    assert_eq!(input.len(), next);
                    assert_eq!(TokenType::Identifier, token.token_type);
                    assert_eq!(op, token.value);
                },
                _ => panic!()
            }
        }
    }
}