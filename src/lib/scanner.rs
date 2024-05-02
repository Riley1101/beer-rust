use crate::objects::tokens::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    end: usize,
    line: usize,
}

impl Scanner {
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
        let c = self.advance();
        match c {
            Some(val) => match val {
                '0'..='9' => {
                    println!("is a number");
                }
                'A'..='z' => {
                    println!("is a char");
                }
                val => {
                    println!("special {}", val);
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
}
