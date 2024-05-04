// TODO better loggin
#![allow(dead_code)]
use crate::objects::tokens::{Token, TokenType};
use std::fmt::Display;

#[derive(Debug)]
pub struct TokenVec {
    tokens: Vec<Token>,
}

impl TokenVec {
    fn new() -> TokenVec {
        TokenVec { tokens: vec![] }
    }
}

impl Display for TokenVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n Vector : {:?}", self)
    }
}

/// The scanner takes in raw source code as a series of characters and groups it into a series of chunks we call tokens.
#[allow(dead_code)] // TODO removed dead_code
pub struct Scanner {
    /// The source string to be scanned
    source: String,
    /// A collection of tokens generated from source
    tokens: TokenVec,
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
            tokens: TokenVec::new(),
            current: 0,
            start: 0,
            end: 0,
            line: 1,
        }
    }

    /// Setter for source in Scanner
    pub fn set_source(self: &mut Self, source: String) {
        self.source = source;
    }

    /// Start scanning in the source string
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
                ' ' | '\n' => {}
                val => {
                    println!("Unexpected characters {}", val);
                }
            },
            None => (),
        };
    }

    /// Consumes the next character in the source file and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// let next_char = self.advance(); // get a character
    ///     
    /// ```
    fn advance(self: &mut Self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    /// Check if scanning gets the end of source
    ///
    /// # Example
    ///
    /// ```
    /// let next_char = self.advance(); // get a character
    ///     
    /// ```
    fn is_end(self: &Self) -> bool {
        self.current > self.source.len()
    }

    fn add_token(self: &mut Self, token: TokenType) {
        println!("{}", token);
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "source - {} \n tokens - {} \n - {} \n",
            self.source, self.tokens, self.current
        )
    }
}
