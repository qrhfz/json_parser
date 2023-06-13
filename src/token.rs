use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub struct Token<'a> {
    pub line: usize,
    pub index: usize,
    pub token_type: TokenType<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType<'a> {
    String { text: &'a str },
    Number { text: &'a str },

    Colon,
    Comma,

    LeftSquareBracket,
    RightSquareBracket,

    LeftCurlyBracket,
    RightCurlyBracket,

    True,
    False,
    Null,
    Error { message: &'a str },
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::String { text, .. } => write!(f, "STR\"{}\"", text),
            TokenType::Number { text, .. } => write!(f, "NUM\"{}\"", text),
            TokenType::Colon => write!(f, "<:>"),
            TokenType::Comma => write!(f, "<,>"),
            TokenType::LeftSquareBracket => write!(f, "<[>"),
            TokenType::RightSquareBracket => write!(f, "<]>"),
            TokenType::LeftCurlyBracket => write!(f, "<{{>"),
            TokenType::RightCurlyBracket => write!(f, "<}}>"),
            TokenType::True => write!(f, "<TRUE>"),
            TokenType::False => write!(f, "<FALSE>"),
            TokenType::Null => write!(f, "<NULL>"),
            TokenType::Error { message, .. } => write!(f, "ERR\"{}\"", message),
        }
    }
}
