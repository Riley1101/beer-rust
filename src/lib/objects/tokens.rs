#![allow(dead_code)]
use std::fmt::Display;

pub const END_LINE_STR: &str = "\0";

#[derive(Debug)]
pub enum LiteralType {
    STRING,
    NUMBER,
    NONE,
}

#[derive(Debug)]
pub enum TokenType {
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    // end of line
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Token struct for each lexeme
// !TODO try implementing `Drop` traits
#[derive(Debug)]
pub struct Token {
    /// Type of the token
    token_type: TokenType,
    /// Actual string slice in source
    lexeme: String,
    /// Type of literals in the language - e.g:string,number
    literal: LiteralType,
    /// Line number of token to be found
    line: usize,
}

impl Token {
    /// A token struct.
    ///
    /// # Arguments
    ///
    /// * `token_type` - Type of token
    /// * `lexeme` - Literal string slice from source
    /// * `literal` - Type of literals in the language
    /// * `line` - Line number in source code that token found
    ///
    /// # Returns `Token`
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralType, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn lexeme_length(self: Self) -> usize {
        self.lexeme.len()
    }
}

impl<'tok> Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
