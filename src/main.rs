use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
enum TokenType {
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

    Identifier(String),
    Number(i32),
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
    fn new() -> Self {
        Scanner {
            source: String::new(),
            current: 0,
            line: 0,
            tokens: vec![],
            keywords: Keywords::new(),
        }
    }

    fn run_file(&mut self, source: &str) {
        self.source = String::from(source);
        let tokens = self.scan_tokens();
        for (i, t) in tokens.iter().enumerate() {
            println!("{}: {:?}", i, t);
        }
    }

    fn run_prompt(&mut self) {
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
            let result = self.scan_token();
        }

        // TODO: Should EOF be on a separate line?
        self.add_token(TokenType::Eof);
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
        self.tokens.push(Token::new(token_type, self.line));
    }

    fn peek(&mut self) -> char {
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        // TODO: Make sure you exit the two loops safely when the last char is not a newline
        let mut current_c = self.advance();
        let tt: TokenType;

        if is_alpha(current_c) {
            let mut identifier = String::new();
            while is_alpha_numeric(current_c) {
                identifier.push(current_c);
                current_c = self.advance()
            }
            self.backtrack(1);
            if self.keywords.keywords.contains_key(&identifier) {
                tt = self.keywords.keywords.get(&identifier).unwrap().clone();
            } else {
                tt = TokenType::Identifier(identifier);
            }
            self.add_token(tt);
        } else if is_digit(current_c) {
            let mut number = String::new();
            while is_digit(current_c) {
                number.push(current_c);
                current_c = self.advance();
            }
            self.backtrack(1);
            tt = TokenType::Number(number.parse::<i32>().unwrap());
            self.add_token(tt);
        } else {
            // <=, !=, >=, ==
            let r = match current_c {
                ';' => Ok(self.add_token(TokenType::Semicolon)),
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
                        if is_alpha_numeric(current_c) {
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
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut scanner = Scanner::new();

    if args.len() > 1 {
        // TODO: Where do you handle errors?
        let file = std::fs::read_to_string(&args[1]).unwrap();
        scanner.run_file(&file);
        return;
    }

    scanner.run_prompt();
}

// var language = "lox";
