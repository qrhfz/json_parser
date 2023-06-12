use std::fmt::Display;

pub enum Token<'a> {
    String(&'a str),
    Number(&'a str),

    Colon,
    Comma,

    LeftSquareBracket,
    RightSquareBracket,

    LeftCurlyBracket,
    RightCurlyBracket,

    True,
    False,
    Null,
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0.eq(r0),
            (Self::Number(l0), Self::Number(r0)) => l0.eq(r0),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::String(s) => write!(f, "STR\"{}\"", s),
            Token::Number(s) => write!(f, "NUM\"{}\"", s),
            Token::Colon => write!(f, "<:>"),
            Token::Comma => write!(f, "<,>"),
            Token::LeftSquareBracket => write!(f, "<[>"),
            Token::RightSquareBracket => write!(f, "<]>"),
            Token::LeftCurlyBracket => write!(f, "<{{>"),
            Token::RightCurlyBracket => write!(f, "<}}>"),
            Token::True => write!(f, "<TRUE>"),
            Token::False => write!(f, "<FALSE>"),
            Token::Null => write!(f, "<NULL>"),
        }
    }
}
