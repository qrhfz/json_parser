use crate::{
    token::{Token, TokenType},
    tokenizer::Tokenizer,
};
use std::collections::{HashMap, VecDeque};

pub fn parse(source: &str) -> Result<JsonNode, String> {
    JsonParser::new(source).parse()
}

struct JsonParser<'a> {
    tokenizer: Tokenizer<'a>,
    buffer: VecDeque<Token<'a>>,
}

struct JsonError<'a> {
    message: &'a str,
    token: Option<Token<'a>>,
}

impl<'a> JsonParser<'a> {
    pub fn new(source: &'a str) -> JsonParser<'a> {
        JsonParser {
            tokenizer: Tokenizer::new(source),
            buffer: VecDeque::new(),
        }
    }

    pub fn parse(&mut self) -> Result<JsonNode, String> {
        match self.value() {
            Ok(json) => Ok(json),
            Err(e) => match e.token {
                Some(token) => Err(format!(
                    "error: {}, at index: {}, line: {}",
                    e.message, token.index, token.line
                )),
                None => Err(format!("error: {}", e.message)),
            },
        }
    }

    fn value(&mut self) -> Result<JsonNode, JsonError<'a>> {
        let tokenopt = self.advance();
        match tokenopt {
            Some(token) => match token.token_type {
                TokenType::Number { text } => Ok(JsonParser::number(&text)),
                TokenType::String { text } => JsonParser::string(&text),
                TokenType::True => Ok(JsonNode::Bool(true)),
                TokenType::False => Ok(JsonNode::Bool(false)),
                TokenType::Null => Ok(JsonNode::Null),
                TokenType::LeftSquareBracket => self.array(),
                TokenType::LeftCurlyBracket => self.object(),
                TokenType::RightSquareBracket => Err(JsonError {
                    message: "Unexpected ]",
                    token: Some(token),
                }),
                TokenType::RightCurlyBracket => Err(JsonError {
                    message: "Unexpected [",
                    token: Some(token),
                }),
                TokenType::Comma => Err(JsonError {
                    message: "Unexpected comma",
                    token: Some(token),
                }),
                TokenType::Colon => Err(JsonError {
                    message: "Unexpected colon",
                    token: Some(token),
                }),
                TokenType::Error { message, .. } => Err(JsonError {
                    message,
                    token: Some(token),
                }),
            },
            None => Err(JsonError {
                message: "eof",
                token: None,
            }),
        }
    }

    fn object(&mut self) -> Result<JsonNode, JsonError<'a>> {
        let mut obj: HashMap<String, JsonNode> = HashMap::new();
        loop {
            let token = self.advance();
            let string = match token.clone() {
                Some(token) => match token.token_type {
                    TokenType::String { text, .. } => JsonParser::escape(text.clone()),
                    TokenType::RightCurlyBracket { .. } => break,
                    _ => {
                        return Err(JsonError {
                            message: "object key is not string",
                            token: Some(token),
                        })
                    }
                },
                None => {
                    return Err(JsonError {
                        message: "eof",
                        token: None,
                    })
                }
            };

            let key = match string {
                Ok(s) => s,
                Err(_) => {
                    return Err(JsonError {
                        message: "invalid string",
                        token,
                    })
                }
            };

            match self.advance() {
                Some(token) => match token.token_type {
                    TokenType::Colon { .. } => {}
                    _ => {
                        return Err(JsonError {
                            message: "expect :",
                            token: Some(token),
                        })
                    }
                },
                None => {
                    return Err(JsonError {
                        message: "expect :",
                        token: None,
                    })
                }
            }

            let value = match self.value() {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            obj.insert(key, value);

            match self.advance() {
                Some(token) => match token.token_type {
                    TokenType::RightCurlyBracket { .. } => break,
                    TokenType::Comma { .. } => continue,
                    _ => {
                        return Err(JsonError {
                            message: "expected comma or object close",
                            token: None,
                        })
                    }
                },
                None => {
                    return Err(JsonError {
                        message: "unexpected eof",
                        token: None,
                    })
                }
            }
        }

        Ok(JsonNode::Object(obj))
    }

    fn array(&mut self) -> Result<JsonNode, JsonError<'a>> {
        let mut arr: Vec<JsonNode> = vec![];
        loop {
            let token = self.peek();
            if token.is_none() {
                return Err(JsonError {
                    message: "eof",
                    token: None,
                });
            }

            let value = match token.unwrap().token_type {
                TokenType::RightSquareBracket { .. } => {
                    self.advance();
                    break;
                }
                _ => self.value(),
            };

            match value {
                Ok(value) => arr.push(value),
                Err(e) => return Err(e),
            }

            let token = self.advance();

            match token {
                Some(token) => match token.token_type {
                    TokenType::RightSquareBracket { .. } => break,
                    TokenType::Comma { .. } => continue,
                    _ => {
                        // return Err(JsonError {
                        //     message: "expected comma or end of array",
                        //     token: Some(token),
                        // })
                        todo!()
                    }
                },
                None => {
                    return Err(JsonError {
                        message: "unexpected eof",
                        token: None,
                    })
                }
            }
        }

        Ok(JsonNode::Array(arr))
    }

    fn string(s: &str) -> Result<JsonNode, JsonError> {
        match JsonParser::escape(s) {
            Ok(s) => Ok(JsonNode::String(s)),
            Err(message) => Err(JsonError {
                message,
                token: None,
            }),
        }
    }

    fn escape(s: &str) -> Result<String, &str> {
        let mut chars = s.chars().peekable();
        let mut escaped = String::with_capacity(s.len());

        chars.next(); // consume first "

        loop {
            let c = match chars.next() {
                Some(c) => c,
                None => return Err("unexpected string end"),
            };

            if c == '\"' {
                break;
            }

            if c != '\\' {
                escaped.push(c);
                continue;
            }

            match chars.next() {
                Some(c) => match c {
                    '\"' => escaped.push('\"'),
                    '\\' => escaped.push('\\'),
                    '/' => escaped.push('/'),
                    'n' => escaped.push('\n'),
                    'b' => {
                        escaped.pop();
                    }
                    'f' => escaped.push(char::from_u32(0xC).unwrap()),
                    'r' => escaped.push('\r'),
                    't' => escaped.push('\t'),
                    'u' => {
                        let mut hexs = String::with_capacity(4);

                        for _ in 0..4 {
                            match chars.next() {
                                Some(c) => hexs.push(c),
                                None => return Err("unexpected eof"),
                            };
                        }
                        let x = match u32::from_str_radix(&hexs, 16) {
                            Ok(n) => n,
                            Err(_) => return Err("parse \\u error"),
                        };
                        match char::from_u32(x) {
                            Some(c) => escaped.push(c),
                            None => return Err("parse \\u error"),
                        }
                    }
                    _ => unreachable!(),
                },
                None => return Err("invalid token"),
            };
        }

        Ok(escaped)
    }

    fn number(s: &str) -> JsonNode {
        JsonNode::Number(s.parse::<f64>().unwrap())
    }

    fn advance(&mut self) -> Option<Token<'a>> {
        if !self.buffer.is_empty() {
            let token = self.buffer.pop_front();
            return token;
        }

        self.tokenizer.next()
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        let token = self.tokenizer.next();
        match token {
            Some(token) => {
                self.buffer.push_back(token);
                self.buffer.back()
            }
            None => None,
        }
    }
}

pub enum JsonNode {
    String(String),
    Number(f64),
    Array(Vec<JsonNode>),
    Object(HashMap<String, JsonNode>),
    Bool(bool),
    Null,
}

impl JsonNode {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            JsonNode::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&f64> {
        match self {
            JsonNode::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            JsonNode::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<JsonNode>> {
        match self {
            JsonNode::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, JsonNode>> {
        match self {
            JsonNode::Object(map) => Some(map),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            JsonNode::Null => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate stats_alloc;

    use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
    use std::alloc::System;

    #[global_allocator]
    static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

    use super::*;

    #[test]
    fn empty_object() {
        let src = "{}";
        let json = parse(src).unwrap();

        json.as_map().unwrap();
    }

    #[test]
    fn empty_array() {
        let src = "[]";
        let json = parse(src).unwrap();

        json.as_vec().unwrap();
    }

    #[test]
    fn object_with_empty_array() {
        let src = "{\"a\":[]}";
        let json = parse(src).unwrap();

        json.as_map().unwrap().get("a").unwrap().as_vec().unwrap();
    }

    #[test]
    fn it_works() {
        let s = "{\"hel\\\"lo\":[1,true,null,\"\\u263a\"]}";

        let json = parse(s).unwrap();

        let arr = json
            .as_map()
            .unwrap()
            .get("hel\"lo")
            .unwrap()
            .as_vec()
            .unwrap();

        assert_eq!(arr[0].as_number().unwrap(), &1_f64);
        assert_eq!(arr[1].as_bool().unwrap(), &true);
        assert_eq!(arr[2].is_null(), true);
        assert_eq!(arr[3].as_string().unwrap(), "☺");

        // let _ = catch_unwind(|| json.as_bool());
    }

    #[test]
    fn unicode_test() {
        let reg = Region::new(&GLOBAL);

        let _ = parse("[\"abcdefg\",\"abcdefg\",\"abcdefg\"]");
        println!("Stats at 1: {:#?}", reg.change());
    }

    #[test]
    fn json_object() {
        let res = parse("{\"id\":\"2489651045\",\"type\":\"CreateEvent\",\"actor\":{\"id\":665991,\"login\":\"petroav\",\"gravatar_id\":\"\",\"url\":\"https://api.github.com/users/petroav\",\"avatar_url\":\"https://avatars.githubusercontent.com/u/665991?\"},\"repo\":{\"id\":28688495,\"name\":\"petroav/6.828\",\"url\":\"https://api.github.com/repos/petroav/6.828\"},\"payload\":{\"ref\":\"master\",\"ref_type\":\"branch\",\"master_branch\":\"master\",\"description\":\"Solution to homework and assignments from MIT's 6.828 (Operating Systems Engineering). Done in my spare time.\",\"pusher_type\":\"user\"},\"public\":true,\"created_at\":\"2015-01-01T15:00:00Z\"}");

        res.unwrap();
    }
}
