use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,

    Bang, // !
    BangEqual,
    EqualEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    Number(f64),
    String(String),

    And, //and
    Else,
    False,
    Fun,
    Class, // class
    Var,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token { token_type, lexeme, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //f.write_fmt(format_args!("TOKEN"))
        write!(f, "Type: {:?}, Line: {}", self.token_type, self.line)
    }
}
