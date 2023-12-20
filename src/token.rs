#![allow(unused)]

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Unknown token
    ILLEGAL,
    EOF,
    INT,
    // Operators
    ASSIGN,
    PLUS,
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
        _ => TokenType::IDENT,
    }
}
