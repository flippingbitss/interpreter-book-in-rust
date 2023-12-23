#![allow(unused, dead_code)]
use crate::{
    ast::{Expr, Program, Stmt},
    lexer::Lexer,
    token::{self, Token, TokenType},
};

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>) -> Self {
        Parser {
            curr_token: lexer.next_token(),
            peek_token: lexer.next_token(),
            lexer,
            errors: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Result<Program, Vec<String>> {
        let mut stmts = Vec::new();

        while self.curr_token != token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.next_token();
        }
        if self.errors.is_empty() {
            Ok(Program { stmts })
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_stmt(&mut self) -> Option<Stmt<'a>> {
        match self.curr_token.token_type {
            TokenType::LET => self.parse_let_stmt(),
            TokenType::RETURN => self.parse_return_stmt(),
            _ => None,
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt<'a>> {
        let token = self.curr_token;
        if !self.advance_if_peek(TokenType::IDENT) {
            return None;
        }
        let name = crate::ast::Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        };

        if !self.advance_if_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.is_curr_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Stmt::Let { name, token })
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt<'a>> {
        let token = self.curr_token;

        while !self.is_curr_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Stmt::Return { token })
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt<'a>> {
        let token = self.curr_token;
        while !self.is_curr_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Stmt::Expr {
            expr: Expr { token },
        })
    }

    fn is_curr_token(&self, tok_type: TokenType) -> bool {
        self.curr_token.token_type == tok_type
    }

    fn is_peek_token(&self, tok_type: TokenType) -> bool {
        self.peek_token.token_type == tok_type
    }

    fn advance_if_peek(&mut self, tok_type: TokenType) -> bool {
        if self.is_peek_token(tok_type) {
            self.next_token();
            true
        } else {
            self.add_error(tok_type);
            false
        }
    }

    fn add_error(&mut self, expect: TokenType) {
        let error_msg = format!(
            "expected next token to be {:?}, instead got {:?}",
            expect, self.peek_token.token_type
        );
        self.errors.push(error_msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expr, Identifier, Stmt},
        lexer::Lexer,
        token::{Token, TokenType},
    };

    use super::Parser;

    #[test]
    fn test_stmts() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;
return 123;
return xyz;
";
        let mut l = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(l);
        let prog = parser.parse();
        assert!(prog.is_ok());

        let expected_lets: Vec<&[u8]> = vec![b"x", b"y", b"foobar"];
        if let Ok(p) = prog {
            for (ident, stmt) in expected_lets.iter().zip(p.stmts.iter()) {
                test_let_stmt(stmt, ident);
            }

            for stmt in p.stmts.iter().skip(3) {
                assert!(matches!(stmt, Stmt::Return { .. }));
            }
        }
    }

    fn test_let_stmt(stmt: &Stmt, tc_name: &[u8]) {
        assert!(matches!(stmt, Stmt::Let { .. }));

        if let Stmt::Let { name, token } = stmt {
            assert_eq!(token.literal, b"let");
            assert_eq!(name.value, tc_name);
        };
    }

    #[test]
    fn test_errors() {
        let input = "
            let x 5;
            let = 10;
            let 123;
            ";

        let mut p = Parser::new(Lexer::new(input.as_bytes()));
        let prog = p.parse();
        assert!(!prog.is_ok());
        log_errors(&prog.err().unwrap());
        assert!(p.errors.len() == 3);
    }

    #[test]
    fn test_display() {
        let a = Stmt::Let {
            token: Token::new(TokenType::LET, b"let"),
            name: Identifier {
                token: Token::new(TokenType::IDENT, b"x"),
                value: b"x",
            },
        };
        eprintln!("{a}");
        assert_eq!(format!("{a}"), "let x = <expr>;");
    }

    fn log_errors(p: &[String]) {
        eprintln!("parser has {} errors", p.len());
        for err in p.iter() {
            eprintln!("parser error: {}", err)
        }
    }

}
