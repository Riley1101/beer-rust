use crate::objects::tokens::{Token, TokenType};

/// The scanner takes in raw source code as a series of characters and groups it into a series of chunks we call tokens.
#[allow(dead_code)] // TODO removed dead_code
pub struct Scanner {
    /// The source string to be scanned
    source: String,
    /// A collection of tokens generated from source
    tokens: Vec<Token>,
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
    pub fn new() -> Self {
        Scanner {
            source: String::new(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            end: 0,
            line: 1,
        }
    }

    pub fn set_tokens(self: &mut Self, source: String) {
        self.source = source;
    }

    pub fn scan_tokens(self: &mut Self) {
        while !self.is_end() {
            self.scan_token();
        }
    }

    pub fn scan_token(self: &mut Self) {
        let c = self.advance();
        match c {
            Some(val) => match val {
                '(' => {
                    self.add_token(TokenType::LEFTPAREN);
                }
                ')' => {
                    self.add_token(TokenType::RIGHTPAREN);
                }
                '{' => {
                    self.add_token(TokenType::LEFTBRACE);
                }
                '}' => {
                    self.add_token(TokenType::RIGHTPAREN);
                }
                ',' => {
                    self.add_token(TokenType::COMMA);
                }
                '.' => {
                    self.add_token(TokenType::DOT);
                }
                '-' => {
                    self.add_token(TokenType::MINUS);
                }
                '+' => {
                    self.add_token(TokenType::PLUS);
                }
                ';' => {
                    self.add_token(TokenType::SEMICOLON);
                }
                '*' => {
                    self.add_token(TokenType::STAR);
                }
                '=' => {
                    self.add_token(TokenType::EQUAL);
                }
                '0'..='9' => {
                    println!("is a number");
                }
                'A'..='z' => {
                    println!("is a char");
                }
                // Escape space and new lines
                ' ' | '\n' => {}
                val => {
                    println!("Unexpected characters {}", val);
                }
            },
            None => (),
        };
    }

    fn advance(self: &mut Self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn is_end(self: &Self) -> bool {
        self.current > self.source.len()
    }

    fn add_token(self: &mut Self, token: TokenType) {
        println!("{}", token);
    }
}
