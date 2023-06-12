use crate::token::Token;

pub struct Tokenizer<'a> {
    start: usize,
    current: usize,
    line: usize,
    src: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Tokenizer {
        Tokenizer {
            start: 0,
            current: 0,
            line: 1,
            src,
        }
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        self.skip_white_spaces();

        // NUMBER
        if self.check_byte(b'-') || self.is_digit() {
            return Some(self.number());
        }

        // STRING
        if self.check_byte(b'"') {
            return Some(self.string());
        }
        let c = self.peek();
        if c.is_none() {
            return None;
        }

        let index = self.current;
        match c.unwrap() {
            b'{' => {
                self.advance();
                Some(Token::LeftCurlyBracket {
                    line: self.line,
                    index,
                })
            }
            b'}' => {
                self.advance();
                Some(Token::RightCurlyBracket {
                    line: self.line,
                    index,
                })
            }
            b'[' => {
                self.advance();
                Some(Token::LeftSquareBracket {
                    line: self.line,
                    index,
                })
            }
            b']' => {
                self.advance();
                Some(Token::RightSquareBracket {
                    line: self.line,
                    index,
                })
            }
            b':' => {
                self.advance();
                Some(Token::Colon {
                    line: self.line,
                    index,
                })
            }
            b',' => {
                self.advance();
                Some(Token::Comma {
                    line: self.line,
                    index,
                })
            }
            b't' => {
                if self.check("true") {
                    self.current += 4;
                    Some(Token::True {
                        line: self.line,
                        index,
                    })
                } else {
                    Some(self.unknown_keyword())
                }
            }
            b'f' => {
                if self.check("false") {
                    self.current += 5;
                    Some(Token::False {
                        line: self.line,
                        index,
                    })
                } else {
                    Some(self.unknown_keyword())
                }
            }

            b'n' => {
                if self.check("null") {
                    self.current += 4;
                    Some(Token::Null {
                        line: self.line,
                        index,
                    })
                } else {
                    Some(self.unknown_keyword())
                }
            }
            _ => Some(self.unknown_keyword()),
        }
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
            return Token::Number {
                line: self.line,
                index: self.start,
                text: &self.src[self.start..self.current],
            };
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

        Token::Number {
            line: self.line,
            index: self.start,
            text: &self.src[self.start..self.current],
        }
    }

    fn string(&mut self) -> Token<'a> {
        self.start = self.current;
        self.advance(); // consume the "

        while !self.at_end() {
            if self.check_byte(b'"') {
                self.advance();
                return Token::String {
                    line: self.line,
                    index: self.start,
                    text: &self.src[self.start..self.current],
                };
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

        Token::Error {
            line: self.line,
            index: self.start,
            message: "unterminated string",
        }
    }

    fn skip_white_spaces(&mut self) {
        while self.current < self.src.len() {
            if !self.is_space() {
                break;
            }
            self.current += 1;
        }
    }

    fn unknown_keyword(&mut self) -> Token<'a> {
        self.start = self.current;
        while !self.at_end() {
            let c = self.peek();

            if c.is_none() {
                break;
            }

            if self.is_space() {
                break;
            }

            match c.unwrap() {
                b'{' | b'}' | b'[' | b']' | b',' | b':' => {
                    break;
                }
                _ => self.advance(),
            }
        }

        return Token::Error {
            line: self.line,
            index: self.start,
            message: "unknown keyword",
        };
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
        let expected = Token::Number {
            line: 1,
            index: 0,
            text: "1234",
        };
        let actual = Tokenizer::new("1234").next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn number_with_spaces() {
        let expected = Token::Number {
            line: 1,
            index: 4,
            text: "1234",
        };
        let actual = Tokenizer::new("    1234    ").next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn number_with_fraction() {
        let expected = Token::Number {
            line: 1,
            index: 0,
            text: "1234.5678",
        };
        let actual = Tokenizer::new("1234.5678").next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn number_with_exponent() {
        let expected = Token::Number {
            line: 1,
            index: 0,
            text: "1234.5678E9",
        };
        let actual = Tokenizer::new("1234.5678E9").next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn number_with_positive_sign_exponent() {
        let expected = Token::Number {
            line: 1,
            index: 0,
            text: "1234.5678E+9",
        };
        let actual = Tokenizer::new("1234.5678E+9").next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn number_with_negative_sign_exponent() {
        let expected = Token::Number {
            line: 1,
            index: 0,
            text: "1234.5678E-9",
        };
        let actual = Tokenizer::new("1234.5678E-9").next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn string() {
        let expected = Token::String {
            index: 0,
            line: 1,
            text: r#""string""#,
        };
        let actual = Tokenizer::new(r#""string""#).next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn unterminated_string() {
        let expected = Token::Error {
            index: 0,
            line: 1,
            message: "unterminated string",
        };
        let actual = Tokenizer::new(r#""string"#).next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn string_with_inner_quote_mark() {
        let expected = Token::String {
            line: 1,
            index: 0,
            text: r#""abc\"def""#,
        };
        let actual = Tokenizer::new(r#""abc\"def""#).next().unwrap();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn symbols_and_keywords() {
        let expected = vec![
            Token::LeftCurlyBracket { line: 1, index: 0 },
            Token::RightCurlyBracket { line: 1, index: 1 },
            Token::LeftSquareBracket { line: 1, index: 2 },
            Token::RightSquareBracket { line: 1, index: 3 },
            Token::Comma { line: 1, index: 4 },
            Token::Colon { line: 1, index: 5 },
            Token::Null { line: 1, index: 7 },
            Token::True { line: 1, index: 12 },
            Token::False { line: 1, index: 17 },
        ];
        let mut tokenizer = Tokenizer::new("{}[],: null true false");

        let mut actual = vec![];
        for _ in 0..expected.len() {
            actual.push(tokenizer.next().unwrap());
        }
        vecs_eq(&actual, &expected);
    }

    fn vecs_eq<T: PartialEq + std::fmt::Debug>(a: &Vec<T>, b: &Vec<T>) {
        assert_eq!(a.len(), b.len());

        for i in 0..a.len() {
            assert_eq!(a[i], b[i]);
        }
    }
}
