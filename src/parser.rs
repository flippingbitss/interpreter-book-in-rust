#![allow(unused, dead_code)]

enum PREC {
    Lowest,
    Equals,
    LtOrGt,
    Sum,
    Product,
    Prefix,
    FnCall,
}

use crate::{
    ast::{self, Expr, Program, Stmt},
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
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt<'a>> {
        let token = self.curr_token;
        if !self.advance_if_peek(TokenType::IDENT) {
            return None;
        }
        let name = ast::Expr::Identifier {
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
        let expr = self.parse_expr(PREC::Lowest as u8);

        // optional semicolon
        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        expr.map(|e| Stmt::Expr { expr: e })
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

    fn parse_expr(&mut self, prec: u8) -> Option<Expr<'a>> {
        // try as prefix first
        let ident = match self.curr_token.token_type {
            TokenType::IDENT => Some(self.parse_ident()),
            TokenType::INT => self.parse_int_literal(),
            _ => None,
        };
        ident
    }

    fn parse_ident(&self) -> Expr<'a> {
        Expr::Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        }
    }

    fn parse_int_literal(&self) -> Option<Expr<'a>> {
        std::str::from_utf8(self.curr_token.literal)
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
            .map(|value| Expr::IntLiteral {
                token: self.curr_token,
                value,
            })
    }

    //fn parse_prefix_expr(&mut self, prec: u8) -> Option<Expr<'a>>
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expr, Program, Stmt},
        lexer::Lexer,
        parser::PREC,
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
            assert!(matches!(name, Expr::Identifier { token, value }));
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
            name: Expr::Identifier {
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

    #[test]
    fn test_expr_stmt() {
        let input = "foobar;";
        assert_prog(input, |stmts| {
            assert_eq!(stmts.len(), 1);
            matches!(stmts[0], Stmt:: Expr { expr: Expr::Identifier {token,..}} if token.literal == b"foobar");
        });
    }

    #[test]
    fn test_int_expr() {
        let input = "123";
        assert_prog(input, |stmts| {
            assert_eq!(stmts.len(), 1);
            matches!(stmts[0], Stmt:: Expr { expr: Expr::IntLiteral {value,..}} if value == 123_i64 );
        });
    }

    fn assert_prog(input: &str, assertions: fn(stmts: &[Stmt])) {
        let mut p = Parser::new(Lexer::new(input.as_bytes()));
        let prog = p.parse();
        assert!(prog.is_ok());
        assertions(&prog.unwrap().stmts);
    }

    #[test]
    fn test_1() {
        dbg!(std::mem::discriminant(&PREC::Lowest));
        dbg!(std::mem::discriminant(&PREC::FnCall));
        dbg!(std::mem::size_of::<PREC>());
        dbg!(PREC::Lowest as u8);
        dbg!(PREC::FnCall as u8);
        //panic!()
    }
}
