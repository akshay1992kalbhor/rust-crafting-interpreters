use super::token::{Token, TokenType};
use super::Keywords;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: Keywords,
}

#[derive(Debug, Clone)]
struct ScannerError {
    line: usize,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected character")
    }
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: String::new(),
            start: 0,
            current: 0,
            line: 0,
            tokens: vec![],
            keywords: Keywords::new(),
        }
    }

    pub fn run_file(&mut self, source: &str) {
        self.source = String::from(source);
        let tokens = self.scan_tokens();
        for (i, t) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, t);
        }
    }

    pub fn run_prompt(&mut self) {
        println!("Running Lox Interpreter v1.0");
        let mut input = String::new();
        loop {
            print!(">> ");
            std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed");
            std::io::stdin().read_line(&mut input).unwrap();
            print!("You typed: {}", input);
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            let result = self.scan_token_classic();
            match result {
                Ok(_) => continue,
                Err(ScannerError { line, .. }) => {
                    println!("Found an unexpected char on line {}", line);
                    return vec![];
                }
            }
        }

        // TODO: Should EOF be on a separate line?
        self.tokens.push(Token::new(TokenType::Eof, String::new(), self.line));
        return self.tokens.clone();
    }

    fn advance(&mut self) -> char {
        // TODO: Substitute advance in the right places
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn backtrack(&mut self, amt: usize) {
        self.current -= amt;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start .. self.current].to_string();
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn scan_token_classic(&mut self) -> Result<(), ScannerError> {
        let mut current_char = self.advance();
        match current_char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.is_expected('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            },
            '=' => {
                let token = if self.is_expected('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            },
            '<' => {
                let token = if self.is_expected('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            },
            '>' => {
                let token = if self.is_expected('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            },
            '/' => {
                if self.is_expected('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            '"' => self.scan_string(),
            '0'..='9' => self.scan_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),
            _ => (),
        }
        Ok(())
    }

    fn scan_identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let key = self.source[self.start .. self.current].to_string();
        if let Some(token_type) = self.keywords.keywords.get(&key) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        match c {
            '0'..='9' |
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    fn scan_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let literal = str::parse::<f64>(&self.source[self.start..self.current]).unwrap();
        self.add_token(TokenType::Number(literal))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // Unterminated string error
        }

        self.advance();
        let literal = self.source[(self.start + 1)..(self.current-1)].to_string();
        self.add_token(TokenType::String(literal));
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        // TODO: Make sure you exit the two loops safely when the last char is not a newline
        let mut current_c = self.advance();
        let tt: TokenType;

        if current_c.is_alphabetic() {
            let mut identifier = String::new();
            while current_c.is_alphanumeric() {
                identifier.push(current_c);
                current_c = self.advance()
            }
            self.backtrack(1);
            if self.keywords.keywords.contains_key(&identifier) {
                tt = self.keywords.keywords.get(&identifier).unwrap().clone();
            } else {
                tt = TokenType::Identifier;
            }
            self.add_token(tt);
        } else if current_c.is_ascii_digit() {
            let mut number = String::new();
            while current_c.is_ascii_digit() {
                number.push(current_c);
                current_c = self.advance();
            }
            self.backtrack(1);
            tt = TokenType::Number(number.parse::<f64>().unwrap());
            self.add_token(tt);
        } else {
            // <=, !=, >=, ==
            let r = match current_c {
                ';' => Ok(self.add_token(TokenType::Semicolon)),
                '+' => Ok(self.add_token(TokenType::Plus)),
                '-' => Ok(self.add_token(TokenType::Minus)),
                '*' => Ok(self.add_token(TokenType::Star)),
                '/' => Ok(self.add_token(TokenType::Slash)),
                '=' | '<' | '>' | '!' => {
                    if self.peek() == '=' {
                        let r1 = match current_c {
                            '=' => Ok(self.add_token(TokenType::EqualEqual)),
                            '<' => Ok(self.add_token(TokenType::LessEqual)),
                            '>' => Ok(self.add_token(TokenType::GreaterEqual)),
                            '!' => Ok(self.add_token(TokenType::BangEqual)),
                            _ => Err(ScannerError { line: self.line }),
                        };
                        r1
                    } else {
                        match current_c {
                            '=' => self.add_token(TokenType::Equal),
                            '<' => self.add_token(TokenType::Less),
                            '>' => self.add_token(TokenType::Greater),
                            '!' => self.add_token(TokenType::Bang),
                            _ => unreachable!(),
                        }
                        Ok(())
                    }
                }
                ' ' | '\t' => Ok(()),
                '\n' => Ok(self.line += 1),
                '"' => {
                    // TODO: Add support for other ascii characters
                    let mut identifier = String::new();
                    loop {
                        current_c = self.advance();
                        if current_c.is_alphanumeric() {
                            identifier.push(current_c);
                        } else if current_c == '"' {
                            break;
                        } else {
                            return Err(ScannerError { line: self.line });
                        }
                    }
                    tt = TokenType::String(identifier);
                    self.add_token(tt);
                    Ok(())
                }
                _ => Err(ScannerError { line: self.line }),
            };
            return r;
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }
}