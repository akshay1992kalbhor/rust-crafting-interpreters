use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
enum TokenType {
    LeftParen,
    RightParen,
    Semicolon,
    DoubleQuotes,

    Equal,
    Greater,

    Identifier(String),
    Number(i32),

    And,
    Else,
    Class,
    Var,

    Eof,
}

// Singleton
struct Keywords {
    keywords: HashMap<String, TokenType>,
}

impl Keywords {
    fn new() -> Self {
        let mut keywords = HashMap::<String, TokenType>::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("var"), TokenType::Var);
        Self { keywords }
    }
}

// HELPERS
fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

/*
impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            line
        )
    }
}
*/

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, line: usize) -> Self {
        Token { token_type, line }
    }
}

use std::fmt;
impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("TOKEN"))

        /*write!(
            f,
            "Type: {:?}, Lexeme: {}, Line: {}",
            self.token_type, self.lexeme, self.line
        )*/
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    keywords: Keywords,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: String::from(source),
            current: 0,
            line: 0,
            tokens: vec![],
            keywords: Keywords::new(),
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            let _ = self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, self.line));
        return self.tokens.clone();
    }

    fn scan_token(&mut self) -> usize {
        let mut current_c = self.source.chars().nth(self.current).unwrap();
        let mut total_consumed = 0usize;

        if is_alpha(current_c) {
            let mut identifier = String::new();
            while is_alpha_numeric(current_c) {
                total_consumed += 1;
                identifier.push(current_c);
                self.current += 1;
                current_c = self.source.chars().nth(self.current).unwrap();
                //println!("{:?}", current_c);
            }
            let t: Token;
            if self.keywords.keywords.contains_key(&identifier) {
                t = Token::new(
                    self.keywords.keywords.get(&identifier).unwrap().clone(),
                    self.line,
                );
            } else {
                t = Token::new(TokenType::Identifier(identifier), self.line);
            }
            self.tokens.push(t);
        } else if is_digit(current_c) {
            let mut number = String::new();
            while is_digit(current_c) {
                total_consumed += 1;
                number.push(current_c);
                self.current += 1;
                current_c = self.source.chars().nth(self.current).unwrap();
            }
            self.tokens.push(Token::new(
                TokenType::Number(number.parse::<i32>().unwrap()),
                self.line,
            ));
        } else {
            total_consumed += 1;
            self.current += 1;
            match current_c {
                '"' => self
                    .tokens
                    .push(Token::new(TokenType::DoubleQuotes, self.line)),
                ';' => self
                    .tokens
                    .push(Token::new(TokenType::Semicolon, self.line)),
                '=' => self.tokens.push(Token::new(TokenType::Equal, self.line)),
                ' ' | '\n' | '\t' => (),
                _ => unreachable!("Invalid token"),
            }
        }
        total_consumed
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert!(args.len() > 1);

    let file = std::fs::read_to_string(&args[1]).unwrap();
    let mut scanner = Scanner::new(&file);
    let tokens = scanner.scan_tokens();

    for (i, t) in tokens.iter().enumerate() {
        println!("{}: {:?}", i, t);
    }
    /*let t = Token {
        token_type: TokenType::String(String::from("lox")),
        line: 0,
        lexeme: String::from("lox"),
    };*/
}

// var language = "lox";
