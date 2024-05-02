use crate::objects::tokens::Token;

#[derive(Debug)]
struct Scanner {
    input: String,
    tokens: Vec<Token>,
}
