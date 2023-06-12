use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token<'a> {
    String {
        line: usize,
        index: usize,
        text: &'a str,
    },
    Number {
        line: usize,
        index: usize,
        text: &'a str,
    },

    Colon {
        line: usize,
        index: usize,
    },
    Comma {
        line: usize,
        index: usize,
    },

    LeftSquareBracket {
        line: usize,
        index: usize,
    },
    RightSquareBracket {
        line: usize,
        index: usize,
    },

    LeftCurlyBracket {
        line: usize,
        index: usize,
    },
    RightCurlyBracket {
        line: usize,
        index: usize,
    },

    True {
        line: usize,
        index: usize,
    },
    False {
        line: usize,
        index: usize,
    },
    Null {
        line: usize,
        index: usize,
    },
    Error {
        line: usize,
        index: usize,
        message: &'a str,
    },
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::String { text, .. } => write!(f, "STR\"{}\"", text),
            Token::Number { text, .. } => write!(f, "NUM\"{}\"", text),
            Token::Colon { .. } => write!(f, "<:>"),
            Token::Comma { .. } => write!(f, "<,>"),
            Token::LeftSquareBracket { .. } => write!(f, "<[>"),
            Token::RightSquareBracket { .. } => write!(f, "<]>"),
            Token::LeftCurlyBracket { .. } => write!(f, "<{{>"),
            Token::RightCurlyBracket { .. } => write!(f, "<}}>"),
            Token::True { .. } => write!(f, "<TRUE>"),
            Token::False { .. } => write!(f, "<FALSE>"),
            Token::Null { .. } => write!(f, "<NULL>"),
            Token::Error { message, .. } => write!(f, "ERR\"{}\"", message),
        }
    }
}
