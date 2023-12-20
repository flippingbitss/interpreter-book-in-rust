#![allow(unused)]

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Unknown token
    ILLEGAL,
    EOF,
    INT,
    // Operators
    BANG,
    MINUS,
    FSLASH,
    MUL,
    LT,
    GT,
    ASSIGN,
    PLUS,
    EQ,
    NOT_EQ,
    // Delimiters
    COMMA,
    SEMICOLON,
    // Misc
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Identifiers + literals
    IDENT,
    FUNCTION,
    LET,
    IF,
    RETURN,
    TRUE,
    ELSE,
    FALSE,
}

pub type Literal<'a> = &'a [u8];

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a [u8],
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, literal: Literal<'a>) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

pub fn lookup_ident(ident: &[u8]) -> TokenType {
    match ident {
        b"fn" => TokenType::FUNCTION,
        b"let" => TokenType::LET,
        b"true" => TokenType::TRUE,
        b"false" => TokenType::FALSE,
        b"return" => TokenType::RETURN,
        b"if" => TokenType::IF,
        b"else" => TokenType::ELSE,
        _ => TokenType::IDENT,
    }
}
