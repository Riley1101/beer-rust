#![allow(dead_code)]
use std::fmt::Display;

#[derive(Debug,Clone, Copy)]
pub enum LiteralType {
    STRING,
    NUMBER,
    NONE,
}

#[derive(Debug,Clone, Copy)]
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
#[derive(Debug,Clone, Copy)]
pub struct Token<'tok> {
    /// Type of the token
    token_type: TokenType,
    /// Actual string slice in source
    lexeme: &'tok str,
    /// Type of literals in the language - e.g:string,number
    literal: LiteralType,
    /// Line number of token to be found
    line: usize,
}

impl<'tok> Token<'tok> {
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
    pub fn new(
        token_type: TokenType,
        lexeme: &'tok str,
        literal: LiteralType,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl<'tok> Display for Token<'tok> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
