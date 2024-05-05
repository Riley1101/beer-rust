use crate::objects::tokens::{LiteralType, Token, TokenType};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TokenVec<'scn> {
    tokens: Vec<Token<'scn>>,
}

impl<'scn> TokenVec<'scn> {
    fn new() -> TokenVec<'scn> {
        TokenVec { tokens: vec![] }
    }

    pub fn push(self: &mut Self, token: Token<'scn>) {
        self.tokens.push(token);
    }
}

impl<'scn> Display for TokenVec<'scn> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n Vector : {:?}", self)
    }
}

/// The scanner takes in raw source code as a series of characters and groups it into a series of chunks we call tokens.
#[allow(dead_code)] // TODO removed dead_code
pub struct Scanner<'scn> {
    /// The source string to be scanned
    source: &'scn String,
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

impl<'scn> Scanner<'scn> {
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
    pub fn new(source: &'scn String) -> Self {
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
    pub fn create_token(&'scn self, char: &Option<char>) -> Option<Token> {
        let token = match char {
            Some(value) => match value {
                '=' => {
                    let lexeme = &self.source[self.start..self.current];
                    let token = Token::new(TokenType::EQUAL, lexeme, LiteralType::NONE, self.line);
                    Some(token)
                }
                '+' => {
                    let lexeme = &self.source[self.start..self.current];
                    let token = Token::new(TokenType::EQUAL, lexeme, LiteralType::NONE, self.line);
                    Some(token)
                }
                _ => None,
            },
            _ => None,
        };
        token
    }

    /// Consumes the next character in the source file and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// let next_char = self.advance(); // get a character
    ///     
    /// ```
    pub fn get_current_char(self: &Self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
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
    pub fn is_end(self: &'scn Self) -> bool {
        self.current > self.source.len()
    }
}

impl<'scn> Display for Scanner<'scn> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "source - {} \n current - {} \n",
            self.source, self.current
        )
    }
}

pub fn scan_tokens<'scn>(input: &String) {
    let mut scanner = Scanner::new(&input);
    while !scanner.is_end() {
        let c = scanner.get_current_char();
        scanner.forward();
        let t = scanner.create_token(&c);
        match t {
            Some(val) => {
                println!("{:?}", val);
                scanner.sync();
            }
            _ => {}
        }
        scanner.sync();
    }
}
