#![allow(unused)]

pub const EOF: Token = Token::new(TokenType::EOF, b"\0");

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

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} (\"{}\")",
            self.token_type,
            std::str::from_utf8(self.literal).unwrap()
        )
    }
}

pub type Literal<'a> = &'a [u8];

#[derive(Copy, Clone, PartialEq)]
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
