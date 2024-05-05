use crate::objects::tokens::{LiteralType, Token, TokenType};
use std::fmt::Display;

#[derive(Debug)]
pub struct TokenVec {
    tokens: Vec<Token>,
}

impl TokenVec {
    fn new() -> TokenVec {
        TokenVec { tokens: vec![] }
    }

    pub fn push(self: &mut Self, token: Token) {
        self.tokens.push(token);
    }
}

impl<'scn> Display for TokenVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n {:#?}", self)
    }
}

/// The scanner takes in raw source code as a series of characters and groups it into a series of chunks we call tokens.
#[allow(dead_code)] // TODO removed dead_code
pub struct Scanner {
    /// The source string to be scanned
    source: String,
    /// A collection of tokens generated from source
    /// The current position of the scanner pointer
    current: usize,
    /// The start index to start the scan
    start: usize,
    /// The end index of token after being scanned
    end: usize,
    /// The line number in the source code where token is found.
    line: usize,
}

impl Scanner {
    /// Creates a new Scanner instance with the provided source code.
    ///
    /// # Arguments
    ///
    /// * `source` - A string containing the source code to be scanned.
    ///
    /// # Returns
    ///
    /// A Scanner instance initialized with the given source code.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::scanner::Scanner;
    ///
    /// let source_code = "fn main() { println!(\"Hello, world!\"); }";
    /// let mut scanner = Scanner::new();
    /// scanner.set_tokens(source_code.to_string());
    /// scanner.scan_tokens();
    ///
    /// ```
    pub fn new(source: String) -> Self {
        let s = Scanner {
            source,
            current: 0,
            start: 0,
            end: 0,
            line: 1,
        };
        s
    }

    pub fn sync(self: &mut Self) {
        self.start = self.current;
    }

    pub fn forward(self: &mut Self) {
        self.current += 1;
    }

    /// Scan tokens
    pub fn scan_token(&mut self, char: &Option<char>) -> Option<Token> {
        let token = match char {
            Some(value) => match value {
                '=' => {
                    let token_type = if self.match_next("=") {
                        TokenType::EQUALEQUAL
                    } else {
                        TokenType::EQUAL
                    };
                    let token = self.build_token(token_type, LiteralType::NONE);
                    Some(token)
                }
                '+' => {
                    let token = self.build_token(TokenType::PLUS, LiteralType::NONE);
                    Some(token)
                }
                '-' => {
                    let token = self.build_token(TokenType::MINUS, LiteralType::NONE);
                    Some(token)
                }
                '*' => {
                    let token = self.build_token(TokenType::STAR, LiteralType::NONE);
                    Some(token)
                }
                '(' => {
                    let token = self.build_token(TokenType::LEFTPAREN, LiteralType::NONE);
                    Some(token)
                }
                ')' => {
                    let token = self.build_token(TokenType::RIGHTPAREN, LiteralType::NONE);
                    Some(token)
                }
                '{' => {
                    let token = self.build_token(TokenType::LEFTBRACE, LiteralType::NONE);
                    Some(token)
                }
                '}' => {
                    let token = self.build_token(TokenType::RIGHTBRACE, LiteralType::NONE);
                    Some(token)
                }
                ',' => {
                    let token = self.build_token(TokenType::COMMA, LiteralType::NONE);
                    Some(token)
                }
                '.' => {
                    let token = self.build_token(TokenType::DOT, LiteralType::NONE);
                    Some(token)
                }
                '0'..='9' => None,
                'A'..='z' => None,
                _ => None,
            },
            _ => None,
        };
        token
    }

    /// Get character at current location
    ///
    /// # Example
    ///
    /// ```
    /// let next_char = self.get_current_char();
    ///     
    /// ```
    pub fn get_current_char(self: &Self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        c
    }

    /// Check if  the end of source string
    ///
    pub fn is_end(self: &Self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn build_token(&self, token_type: TokenType, literal: LiteralType) -> Token {
        println!("{:?}", self.source.chars());
        println!("{} {}", self.start, self.current);
        let lexeme = &self.source[self.start..self.current];
        println!("{} ", lexeme);
        let token = Token::new(token_type, lexeme.to_string(), literal, self.line);
        token
    }

    // TODO  !DOING Check the next character to see if they matches
    fn match_next(&mut self, expected: &str) -> bool {
        if self.is_end() {
            return false;
        }
        let char = &self.source[self.current..self.current];
        println!("current char {}", char);
        if char == expected {
            return false;
        }
        self.current += 1;
        true
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "source - {} \n current - {} \n",
            self.source, self.current
        )
    }
}

pub fn scan_tokens(input: String) -> TokenVec {
    let mut token_vec = TokenVec::new();
    let mut scanner = Scanner::new(input);
    while !scanner.is_end() {
        let c = scanner.get_current_char();
        scanner.forward();
        let t = scanner.scan_token(&c);
        match t {
            Some(val) => {
                token_vec.push(val);
                scanner.sync();
            }
            _ => {}
        }
        scanner.sync();
    }
    println!("{token_vec}");
    token_vec
}
