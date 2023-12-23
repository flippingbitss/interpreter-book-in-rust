#![allow(unused, dead_code)]

use crate::token::{self, Token, TokenType as tt};

#[derive(Default)]
pub(crate) struct Lexer<'a> {
    // todo: use &str instead to support utf-8
    // todo: impl Iterator for lexer since Lexer is techinally an iterator 
    // yielding char/byte tokens
    input: &'a [u8],
    pos: usize,
    read_pos: usize,
    ch: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new<I: Into<&'a [u8]>>(input: I) -> Self {
        let mut l = Lexer {
            input: input.into(),
            ..Default::default()
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_ws();
        let ch = self.ch;
        debug_assert_ne!(ch, b" ");
        let mut consume_next = true;
        let tok = match ch[0] {
            b'=' => {
                let next = self.peek_char()[0];
                if next == b'=' {
                    self.read_char();
                    Token::new(tt::EQ, b"==")
                } else {
                    Token::new(tt::ASSIGN, ch)
                }
            }
            b'!' => {
                let next = self.peek_char()[0];
                if next == b'=' {
                    self.read_char();
                    Token::new(tt::NOTEQ, b"!=")
                } else {
                    Token::new(tt::BANG, ch)
                }
            }
            b'+' => Token::new(tt::PLUS, ch),
            b';' => Token::new(tt::SEMICOLON, ch),
            b'(' => Token::new(tt::LPAREN, ch),
            b')' => Token::new(tt::RPAREN, ch),
            b'{' => Token::new(tt::LBRACE, ch),
            b'}' => Token::new(tt::RBRACE, ch),
            b',' => Token::new(tt::COMMA, ch),
            b'-' => Token::new(tt::MINUS, ch),
            b'/' => Token::new(tt::FSLASH, ch),
            b'*' => Token::new(tt::MUL, ch),
            b'<' => Token::new(tt::LT, ch),
            b'>' => Token::new(tt::GT, ch),
            b'\0' => Token::new(tt::EOF, ch),
            c if Self::is_letter(c) => {
                consume_next = false;
                let ident = self.read_ident();
                let ttype = token::lookup_ident(ident);
                Token::new(ttype, ident)
            }
            c if Self::is_digit(c) => {
                consume_next = false;
                let ident = self.read_num();
                Token::new(tt::INT, ident)
            }
            _ => Token::new(tt::ILLEGAL, ch),
        };
        if consume_next {
            self.read_char();
        }
        tok
    }

    fn read_ident(&mut self) -> &'a [u8] {
        let start = self.pos;
        while Self::is_letter(self.ch[0]) {
            self.read_char()
        }
        &self.input[start..self.pos]
    }

    fn read_num(&mut self) -> &'a [u8] {
        let start = self.pos;
        while Self::is_digit(self.ch[0]) {
            self.read_char()
        }
        &self.input[start..self.pos]
    }

    fn read_char(&mut self) {
        let ch = self
            .input
            .get(self.read_pos..(self.read_pos + 1))
            .unwrap_or(b"\0");
        self.pos = self.read_pos;
        self.read_pos += 1;
        self.ch = ch;
    }

    fn peek_char(&mut self) -> &[u8] {
        self.input
            .get(self.read_pos..(self.read_pos + 1))
            .unwrap_or(b"\0")
    }

    fn skip_ws(&mut self) {
        const WS_CHARS: &[u8] = b" \t\n\r";
        while WS_CHARS.contains(&self.ch[0]) {
            self.read_char();
        }
    }

    fn is_letter(ch: u8) -> bool {
        ch.is_ascii_alphabetic() || ch == b'_'
    }

    fn is_digit(ch: u8) -> bool {
        ch.is_ascii_digit()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenType as tt},
    };

    #[test]
    fn test_new_token() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
";

        let tests = [
            Token::new(tt::LET, b"let"),
            Token::new(tt::IDENT, b"five"),
            Token::new(tt::ASSIGN, b"="),
            Token::new(tt::INT, b"5"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::LET, b"let"),
            Token::new(tt::IDENT, b"ten"),
            Token::new(tt::ASSIGN, b"="),
            Token::new(tt::INT, b"10"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::LET, b"let"),
            Token::new(tt::IDENT, b"add"),
            Token::new(tt::ASSIGN, b"="),
            Token::new(tt::FUNCTION, b"fn"),
            Token::new(tt::LPAREN, b"("),
            Token::new(tt::IDENT, b"x"),
            Token::new(tt::COMMA, b","),
            Token::new(tt::IDENT, b"y"),
            Token::new(tt::RPAREN, b")"),
            Token::new(tt::LBRACE, b"{"),
            Token::new(tt::IDENT, b"x"),
            Token::new(tt::PLUS, b"+"),
            Token::new(tt::IDENT, b"y"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::RBRACE, b"}"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::LET, b"let"),
            Token::new(tt::IDENT, b"result"),
            Token::new(tt::ASSIGN, b"="),
            Token::new(tt::IDENT, b"add"),
            Token::new(tt::LPAREN, b"("),
            Token::new(tt::IDENT, b"five"),
            Token::new(tt::COMMA, b","),
            Token::new(tt::IDENT, b"ten"),
            Token::new(tt::RPAREN, b")"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::BANG, b"!"),
            Token::new(tt::MINUS, b"-"),
            Token::new(tt::FSLASH, b"/"),
            Token::new(tt::MUL, b"*"),
            Token::new(tt::INT, b"5"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::INT, b"5"),
            Token::new(tt::LT, b"<"),
            Token::new(tt::INT, b"10"),
            Token::new(tt::GT, b">"),
            Token::new(tt::INT, b"5"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::IF, b"if"),
            Token::new(tt::LPAREN, b"("),
            Token::new(tt::INT, b"5"),
            Token::new(tt::LT, b"<"),
            Token::new(tt::INT, b"10"),
            Token::new(tt::RPAREN, b")"),
            Token::new(tt::LBRACE, b"{"),
            Token::new(tt::RETURN, b"return"),
            Token::new(tt::TRUE, b"true"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::RBRACE, b"}"),
            Token::new(tt::ELSE, b"else"),
            Token::new(tt::LBRACE, b"{"),
            Token::new(tt::RETURN, b"return"),
            Token::new(tt::FALSE, b"false"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::RBRACE, b"}"),
            Token::new(tt::INT, b"10"),
            Token::new(tt::EQ, b"=="),
            Token::new(tt::INT, b"10"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::INT, b"10"),
            Token::new(tt::NOTEQ, b"!="),
            Token::new(tt::INT, b"9"),
            Token::new(tt::SEMICOLON, b";"),
            Token::new(tt::EOF, b"\0"),
        ];

        let mut l = Lexer::new(input.as_bytes());

        for case in tests {
            let tok = l.next_token();
            assert_eq!(
                tok.token_type, case.token_type,
                "Wrong token_type, \nexpected = {:?}, \nprovided = {:?}",
                case.token_type, tok.token_type
            );
            assert_eq!(
                tok.literal, case.literal,
                "Wrong literal, \nexpected = {:?}, \nprovided = {:?}",
                case.literal, tok.literal
            );
            println!(
                "case {:?} success, value = {:?}",
                case.token_type,
                std::str::from_utf8(tok.literal)
            );
        }
    }
}
