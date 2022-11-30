use crate::lexic::{token::{Token, self}, utils};

/// Function to scan a number
/// 
/// This function assumes that the character at `start_pos` is a number [0-9],
/// if not it will panic
pub fn scan(chars: &Vec<char>, start_pos: usize) -> Result<(Token, usize), String> {
    let next_char = chars.get(start_pos);

    // Try to scan a HEX value
    if let Some(c) = next_char {
        if *c == '0' {
            // May be an 'x' or 'X'
            let next_next_char = chars.get(start_pos + 1);
            if let Some(c2) = next_next_char {
                if *c2 == 'x' || *c2 == 'X' {
                    return scan_hex(chars, start_pos + 2, String::from("0x"));
                }
            }
        }
    }


    scan_decimal(chars, start_pos, String::from(""))
}


/// Recursively scans an integer. If a dot `.` is found, scans a double,
/// if a `e` is found, scans a number in scientific notation
fn scan_decimal(chars: &Vec<char>, start_pos: usize, current: String) -> Result<(Token, usize), String> {
    let next_char = chars.get(start_pos);

    
    // If a char is found
    if let Some(c) = next_char {
        let c = *c;

        // If a dot is found scan a double
        if c == '.' {
            return scan_double(chars, start_pos + 1, utils::str_append(current, c))
        }
        // If a `e` is found scan a double with exponent
        else if c == 'e' {
            return scan_scientific(chars, start_pos + 1, utils::str_append(current, c))
        }
        // Scan a decimal number
        else if utils::is_digit(c) {
            return scan_decimal(chars, start_pos + 1, utils::str_append(current, c))
        }
    }

    // Return the current value
    Ok((token::new_number(current, start_pos as i32), start_pos))
}


/// Recursively scans a hex number
/// 
/// This function expects the following on the first call:
/// - The char at `start_pos` is a value between [0-9a-fA-F]. If not, will return an error.
/// - `current == "0x"`. If not will return an incorrect value, or panic.
fn scan_hex(chars: &Vec<char>, start_pos: usize, current: String) -> Result<(Token, usize), String> {
    let next_char = chars.get(start_pos);

    if let Some(c) = next_char {
        let c = *c;

        if utils::is_hex_digit(c) {
            return scan_hex(chars, start_pos + 1, utils::str_append(current, c))
        }
    }

    // If the current value is "0x" that means that there wasn't any hex number thereafter
    if current.len() == 2 {
        Err(String::from("Tried to scan an incomplete hex value"))
    }
    else {
        Ok((token::new_number(current, start_pos as i32), start_pos))
    }
}


/// Scans a floating point number
/// 
/// This function expects the following:
/// - `start_pos` is the position after the dot. E.g., if the input is `3.22` then `start_pos == 2`.
/// 
/// Returns a syntax error if the char at `start_pos` is not a value between [0-9]
fn scan_double(chars: &Vec<char>, start_pos: usize, current: String) -> Result<(Token, usize), String> {
    let next_char = chars.get(start_pos);

    // Check that the first characters exists and is a number
    if let Some(c) = next_char {
        if utils::is_digit(*c) { Ok(scan_digits(chars, start_pos, current)) }
        else {Err(String::from("The character after the dot when scanning a double is not a number."))}
    }
    else {
        Err(String::from("EOF when scanning a double number."))
    }
}


/// Scans a double in scientific notation
/// 
/// This function expects the following:
/// - `start_pos` is the position after the `e`. E.g., if the input is `3e+10` then `start_pos == 2`
/// 
/// Returns a syntax error if:
/// - The char at `start_pos` is not `+` or `-`
/// - The char at `start_pos + 1` is not between [0-9]
fn scan_scientific(chars: &Vec<char>, start_pos: usize, current: String) -> Result<(Token, usize), String> {
    let next_char_1 = chars.get(start_pos);
    let next_char_2 = chars.get(start_pos + 1);

    match (next_char_1, next_char_2) {
        (Some(c1), Some(c2)) if (*c1 == '+' || *c1 == '-') && utils::is_digit(*c2) => {
            let new_value = format!("{}{}{}", current, *c1, *c2);
            Ok(scan_digits(chars, start_pos + 2, new_value))
        },
        _ => Err(String::from("The characters after 'e' are not + or -, or are not followed by a number"))
    }
}


/// Scans chars between [0-9], returns when none is found
fn scan_digits(chars: &Vec<char>, start_pos: usize, current: String) -> (Token, usize) {
    let next_char = chars.get(start_pos);

    if let Some(c) = next_char {
        let c = *c;

        if utils::is_digit(c) {
            return scan_digits(chars, start_pos + 1, utils::str_append(current, c))
        }
    }

    // Return current value
    (token::new_number(current, start_pos as i32), start_pos)
}




#[cfg(test)]
mod tests {
    use crate::lexic::token::TokenType;

    use super::*;

    fn str_to_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_int() {
        let input = str_to_vec("123");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(3, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123", token.value);


        let input = str_to_vec("0123 ");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0123", token.value);


        let input = str_to_vec("  123456 789");
        let start_pos = 2;

        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(8, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123456", token.value);
    }

    // Should not scan whitespace after the number
    #[test]
    fn test_int_2() {
        let input = str_to_vec("123 ");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(3, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123", token.value);
    }

    #[test]
    fn test_hex() {
        let input = str_to_vec("0x20 ");
        let start_pos = 0;

        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0x20", token.value);


        let input = str_to_vec("    0Xff23DA ");
        let start_pos = 4;

        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(12, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0xff23DA", token.value);
    }

    // Should not scan an incomplete hex value
    #[test]
    fn test_hex_2() {
        let input = str_to_vec("0x ");
        let start_pos = 0;

        match scan(&input, start_pos) {
            Ok(_) => panic!(),
            Err(reason) => assert_eq!("Tried to scan an incomplete hex value", reason)
        }


        let input = str_to_vec("0 x20 ");
        let start_pos = 0;
        let (token, _) = scan(&input, start_pos).unwrap();
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0", token.value);
    }

    // Should not scan a hex value if it doesn't start with 0x
    #[test]
    fn test_hex_3() {
        let input = str_to_vec("1x20");
        let start_pos = 0;
        let (token, _) = scan(&input, start_pos).unwrap();
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("1", token.value);
    }

    // Should scan a double
    #[test]
    fn test_double_1() {
        let input = str_to_vec("3.22");
        let start_pos = 0;
        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("3.22", token.value);


        let input = str_to_vec("123456.7890 ");
        let start_pos = 0;
        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(11, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123456.7890", token.value);
    }


    // Should not scan an incomplete double
    #[test]
    fn test_double_2() {
        let input = str_to_vec("322.  ");
        let start_pos = 0;

        match scan(&input, start_pos) {
            Ok(_) => panic!(),
            Err(reason) => assert_eq!("The character after the dot when scanning a double is not a number.", reason)
        }


        let input = str_to_vec("322.");
        let start_pos = 0;

        match scan(&input, start_pos) {
            Ok(_) => panic!(),
            Err(reason) => assert_eq!("EOF when scanning a double number.", reason)
        }
    }

    // Should scan a double without decimal part, with exponent
    #[test]
    fn test_exp_1() {
        let input = str_to_vec("1e+0");
        let start_pos = 0;
        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!("1e+0", token.value);
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);

        let input = str_to_vec("1e-0");
        let start_pos = 0;
        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("1e-0", token.value);


        let input = str_to_vec("0e+0");
        let start_pos = 0;
        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(4, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("0e+0", token.value);

        let input = str_to_vec("123498790e+12349870");
        let start_pos = 0;
        let (token, next) = scan(&input, start_pos).unwrap();
        assert_eq!(19, next);
        assert_eq!(TokenType::Number, token.token_type);
        assert_eq!("123498790e+12349870", token.value);
    }
}
