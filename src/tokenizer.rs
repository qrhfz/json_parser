use std::cmp::{max, min};

use crate::token::Token;

pub struct Tokenizer<'a> {
    start: usize,
    current: usize,
    src: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Tokenizer {
        Tokenizer {
            start: 0,
            current: 0,
            src,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut tokens: Vec<Token> = vec![];

        while self.current < self.src.len() {
            self.skip_white_spaces();

            // NUMBER
            if self.check_byte(b'-') || self.is_digit() {
                let token = self.number();
                tokens.push(token);
                continue;
            }

            // STRING
            if self.check_byte(b'"') {
                let token = self.string();
                tokens.push(token);
                continue;
            }
            let c = self.peek();
            if c.is_none() {
                break;
            }

            match c.unwrap() {
                b'{' => tokens.push(Token::LeftCurlyBracket),
                b'}' => tokens.push(Token::RightCurlyBracket),
                b'[' => tokens.push(Token::LeftSquareBracket),
                b']' => tokens.push(Token::RightSquareBracket),
                b':' => tokens.push(Token::Colon),
                b',' => tokens.push(Token::Comma),
                b't' => {
                    if self.check("true") {
                        self.current += 4;
                        tokens.push(Token::True);
                        continue;
                    }
                    return Err("unexpected token: expect true".to_string());
                }
                b'f' => {
                    if self.check("false") {
                        self.current += 5;
                        tokens.push(Token::False);
                        continue;
                    }
                    return Err("unexpected token: expect false".to_string());
                }
                b'n' => {
                    if self.check("null") {
                        self.current += 4;
                        tokens.push(Token::Null);
                        continue;
                    }
                    return Err("unexpected token: expect null".to_string());
                }
                _ => {
                    return Err(format!(
                        "unexpected token: unknow token: {}",
                        &self.src
                            [max(self.current - 10, 0)..min(self.current + 10, self.src.len())]
                    ));
                }
            }
            self.advance();
        }

        Ok(tokens)
    }

    fn number(&mut self) -> Token<'a> {
        self.start = self.current;
        if self.check_byte(b'-') {
            self.advance(); // consume minus sign
        }

        if self.is_zero() {
            self.advance(); // consume zero
        } else if self.is_1to9() {
            self.advance(); // consume first digit
            while !self.at_end() && self.is_digit() {
                self.advance();
            }
        }

        if self.at_end() {
            return Token::Number(&self.src[self.start..self.current]);
        }

        if self.check_byte(b'.') {
            self.advance(); // consume the dot
            while !self.at_end() && self.is_digit() {
                self.advance();
            }
        }

        if self.check_byte(b'E') || self.check_byte(b'e') {
            self.advance(); // consume the E
            if self.check_byte(b'+') || self.check_byte(b'-') {
                self.advance(); // consume the + or -
            }
            while !self.at_end() && self.is_digit() {
                self.advance();
            }
        }

        Token::Number(&self.src[self.start..self.current])
    }

    fn string(&mut self) -> Token<'a> {
        self.start = self.current;
        self.advance(); // consume the "

        loop {
            if self.check_byte(b'"') {
                self.advance();
                break;
            }

            if self.check_byte(b'\\') {
                self.advance();

                // if self.check_byte(b'"') {
                self.advance();
                // }
                continue;
            }

            self.advance();
        }

        Token::String(&self.src[self.start..self.current])
    }

    fn skip_white_spaces(&mut self) {
        while self.current < self.src.len() {
            if !self.is_space() {
                break;
            }
            self.current += 1;
        }
    }

    fn is_space(&self) -> bool {
        match self.peek() {
            Some(c) => c == b' ' || c == b'\n' || c == b'\t' || c == b'\r',
            None => false,
        }
    }

    fn is_digit(&self) -> bool {
        self.is_zero() || self.is_1to9()
    }

    fn is_1to9(&self) -> bool {
        match self.peek() {
            Some(c) => c >= b'1' && c <= b'9',
            None => false,
        }
    }

    fn is_zero(&self) -> bool {
        match self.peek() {
            Some(c) => c == b'0',
            None => false,
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self) -> Option<u8> {
        if self.at_end() {
            return None;
        }
        return Some(self.src.as_bytes()[self.current]);
    }

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn check_byte(&self, byte: u8) -> bool {
        match self.peek() {
            Some(c) => c == byte,
            None => false,
        }
    }

    fn check(&self, comparison: &str) -> bool {
        if self.current + comparison.len() > self.src.len() {
            return false;
        }
        self.src[self.current..self.current + comparison.len()].eq(comparison)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let expected = vec![Token::Number("1234")];
        let actual = Tokenizer::new("1234").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn number_with_spaces() {
        let expected = vec![Token::Number("1234")];
        let actual = Tokenizer::new("    1234    ").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn number_with_fraction() {
        let expected = vec![Token::Number("1234.5678")];
        let actual = Tokenizer::new("1234.5678").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn number_with_exponent() {
        let expected = vec![Token::Number("1234.5678E9")];
        let actual = Tokenizer::new("1234.5678E9").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn number_with_positive_sign_exponent() {
        let expected = vec![Token::Number("1234.5678E+9")];
        let actual = Tokenizer::new("1234.5678E+9").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn number_with_negative_sign_exponent() {
        let expected = vec![Token::Number("1234.5678E-9")];
        let actual = Tokenizer::new("1234.5678E-9").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn string() {
        let expected = vec![Token::String(r#""string""#)];
        let actual = Tokenizer::new(r#""string""#).tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn string_with_inner_quote_mark() {
        let expected = vec![Token::String(r#""abc\"def""#)];
        let actual = Tokenizer::new(r#""abc\"def""#).tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    #[test]
    fn symbols_and_keywords() {
        let expected = vec![
            Token::LeftCurlyBracket,
            Token::RightCurlyBracket,
            Token::LeftSquareBracket,
            Token::RightSquareBracket,
            Token::Comma,
            Token::Colon,
            Token::Null,
            Token::True,
            Token::False,
        ];
        let actual = Tokenizer::new("{}[],: null true false").tokenize().unwrap();
        assert!(do_vecs_match(&actual, &expected));
    }

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
