#![allow(unused)]

use core::fmt;

pub const EOF: Token = Token::new(TokenType::EOF, b"\0");

#[derive(Debug, Copy, Clone)]
pub enum Prec {
    Lowest,
    Equals,
    LtOrGt,
    Sum,
    Product,
    Prefix,
    FnCall,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    NOTEQ,
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

impl TokenType {
    pub fn precedence(&self) -> Prec {
        match self {
            Self::EQ => Prec::Equals,
            Self::NOTEQ => Prec::Equals,
            Self::LT => Prec::LtOrGt,
            Self::GT => Prec::LtOrGt,
            Self::PLUS => Prec::Sum,
            Self::MINUS => Prec::Sum,
            Self::FSLASH => Prec::Product,
            Self::MUL => Prec::Product,
            _ => Prec::Lowest,
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.literal).unwrap())
    }
}

pub type Literal<'a> = &'a [u8];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a [u8],
}

impl<'a> Token<'a> {
    pub const fn new(token_type: TokenType, literal: Literal<'a>) -> Self {
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
