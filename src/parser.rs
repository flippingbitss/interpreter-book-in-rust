#![allow(unused, dead_code)]

use crate::{
    ast::{self, Expr, Program, Stmt},
    lexer::Lexer,
    token::{self, Prec, Token, TokenType},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token<'a>,
    peek_token: Token<'a>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
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

    pub fn parse(&mut self) -> Result<Program<'a>, Vec<String>> {
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

        self.next_token();

        let value = self.parse_expr(Prec::Lowest);

        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        value.map(|v| Stmt::Let {
            name,
            token,
            value: v,
        })
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt<'a>> {
        let token = self.curr_token;

        self.next_token();
        let value = self.parse_expr(Prec::Lowest);

        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        value.map(|v| Stmt::Return { token, value: v })
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt<'a>> {
        let token = self.curr_token;
        let expr = self.parse_expr(Prec::Lowest);
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

    fn parse_expr(&mut self, prec: Prec) -> Option<Expr<'a>> {
        // try as prefix first
        let curr_tt = self.curr_token.token_type;
        let left = match curr_tt {
            TokenType::FUNCTION => self.parse_fn_literal(),
            TokenType::LPAREN => self.parse_group_expr(),
            TokenType::IF => self.parse_if_expr(),
            TokenType::TRUE => self.parse_bool_literal(),
            TokenType::FALSE => self.parse_bool_literal(),
            TokenType::IDENT => self.parse_ident(),
            TokenType::INT => self.parse_int_literal(),
            TokenType::BANG => self.parse_prefix_expr(prec),
            TokenType::MINUS => self.parse_prefix_expr(prec),
            _ => None,
        };

        if left.is_none() {
            return None;
        }
        let mut expr = left;
        while !self.is_peek_token(TokenType::SEMICOLON)
            && (prec as u8) < self.peek_token.token_type.precedence() as u8
        {
            let infix = matches!(
                self.peek_token.token_type,
                TokenType::PLUS
                    | TokenType::MINUS
                    | TokenType::FSLASH
                    | TokenType::MUL
                    | TokenType::EQ
                    | TokenType::NOTEQ
                    | TokenType::LT
                    | TokenType::GT
            );
            let call = matches!(self.peek_token.token_type, TokenType::LPAREN);
            if infix {
                self.next_token();
                expr = expr.and_then(|e| self.parse_infix_expr(e));
            } else if call {
                self.next_token();
                expr = expr.and_then(|e| self.parse_call_expr(e))
            } else {
                return expr;
            }
        }
        expr
    }

    fn parse_ident(&self) -> Option<Expr<'a>> {
        Some(Expr::Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        })
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

    fn parse_bool_literal(&self) -> Option<Expr<'a>> {
        Some(Expr::BoolLiteral {
            token: self.curr_token,
            value: self.is_curr_token(TokenType::TRUE),
        })
    }

    fn parse_prefix_expr(&mut self, prec: Prec) -> Option<Expr<'a>> {
        let token = self.curr_token;
        self.next_token();
        self.parse_expr(Prec::Prefix).map(|expr| Expr::Prefix {
            token,
            op: token.literal,
            expr: Box::new(expr),
        })
    }

    fn parse_infix_expr(&mut self, left: Expr<'a>) -> Option<Expr<'a>> {
        let token = self.curr_token;
        let curr_prec = token.token_type.precedence();
        self.next_token();
        let expr = self.parse_expr(curr_prec).unwrap();
        Some(Expr::Infix {
            token,
            left: Box::new(left),
            op: token.literal,
            right: Box::new(expr),
        })
    }

    fn parse_group_expr(&mut self) -> Option<Expr<'a>> {
        self.next_token();
        let expr = self.parse_expr(Prec::Lowest);
        if !self.advance_if_peek(TokenType::RPAREN) {
            return None;
        }
        expr
    }

    fn parse_if_expr(&mut self) -> Option<Expr<'a>> {
        let token = self.curr_token;
        if !self.advance_if_peek(TokenType::LPAREN) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expr(Prec::Lowest);

        if condition.is_none() {
            return None;
        }

        if !self.advance_if_peek(TokenType::RPAREN) {
            return None;
        }

        if !self.advance_if_peek(TokenType::LBRACE) {
            return None;
        }

        let consequence = self.parse_block_stmt();
        let mut alternative = None;
        if self.is_peek_token(TokenType::ELSE) {
            self.next_token();
            if !self.advance_if_peek(TokenType::LBRACE) {
                return None;
            }
            alternative = Some(Box::new(self.parse_block_stmt()));
        }

        Some(Expr::If {
            token,
            condition: Box::new(condition.unwrap()),
            consequence: Box::new(consequence),
            alternative,
        })
    }

    fn parse_block_stmt(&mut self) -> Stmt<'a> {
        let token = self.curr_token;
        let mut stmts = Vec::new();
        while !self.is_curr_token(TokenType::RBRACE) && !self.is_curr_token(TokenType::EOF) {
            if let Some(s) = self.parse_stmt() {
                stmts.push(s);
            }
            self.next_token();
        }

        Stmt::Block { token, stmts }
    }

    fn parse_fn_literal(&mut self) -> Option<Expr<'a>> {
        let token = self.curr_token;
        if !self.advance_if_peek(TokenType::LPAREN) {
            return None;
        }

        let parameters = self.parse_fn_parameters();

        if !self.advance_if_peek(TokenType::LBRACE) {
            return None;
        }

        let block = self.parse_block_stmt();

        Some(Expr::FnLiteral {
            token,
            parameters,
            block: Box::new(block),
        })
    }

    fn parse_fn_parameters(&mut self) -> Vec<Expr<'a>> {
        if self.is_peek_token(TokenType::RPAREN) {
            self.next_token();
            return vec![];
        }

        self.next_token();
        let mut params = Vec::new();
        // first param
        let ident = Expr::Identifier {
            token: self.curr_token,
            value: self.curr_token.literal,
        };
        params.push(ident);

        while self.is_peek_token(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let ident = Expr::Identifier {
                token: self.curr_token,
                value: self.curr_token.literal,
            };
            params.push(ident);
        }

        if !self.advance_if_peek(TokenType::RPAREN) {
            return vec![];
        }

        params
    }

    fn parse_call_expr(&mut self, fn_expr: Expr<'a>) -> Option<Expr<'a>> {
        let arguments = self.parse_call_args();
        Some(Expr::Call {
            token: self.curr_token,
            function: Box::new(fn_expr),
            arguments,
        })
    }

    fn parse_call_args(&mut self) -> Vec<Expr<'a>> {
        if self.is_peek_token(TokenType::RPAREN) {
            self.next_token();
            return vec![];
        }
        self.next_token();
        let mut args = Vec::new();
        if let Some(e) = self.parse_expr(Prec::Lowest) {
            args.push(e);
        }
        while self.is_peek_token(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            if let Some(e) = self.parse_expr(Prec::Lowest) {
                args.push(e);
            }
        }
        if !self.advance_if_peek(TokenType::RPAREN) {
            return vec![];
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expr, Program, Stmt},
        lexer::Lexer,
        parser::Prec,
        token::{Token, TokenType},
    };

    use super::Parser;

    type AssertExpr = fn(&Expr) -> ();

    fn bytes_as_str(value: &[u8]) -> &str {
        std::str::from_utf8(value).unwrap()
    }

    fn assert_let_stmt(stmt: &Stmt, ename: &[u8], value_assert: AssertExpr) {
        if let Stmt::Let { name, token, value } = stmt {
            assert_eq!(token.literal, b"let");
            value_assert(value);
        } else {
            panic!("not a let statement")
        }
    }

    fn assert_return_stmt(stmt: &Stmt, value_assert: AssertExpr) {
        if let Stmt::Return { token, value } = stmt {
            assert_eq!(token.literal, b"return");
            value_assert(value);
        } else {
            panic!("not a return statement")
        }
    }

    fn assert_int_literal(expr: &Expr, expected: i64) {
        match expr {
            Expr::IntLiteral { token, value } => {
                assert_eq!(
                    *value, expected,
                    "expected Int value {} but got {}",
                    expected, *value
                );
            }
            _ => {
                panic!("expr not an int literal. Instead got {:?}", expr);
            }
        }
    }

    fn assert_ident(expr: &Expr, expected_literal: &[u8]) {
        match expr {
            Expr::Identifier { token, value } => {
                assert_eq!(bytes_as_str(value), bytes_as_str(expected_literal))
            }
            _ => {
                panic!("expr not an int literal. Instead got {:?}", expr);
            }
        }
    }

    fn assert_expr_stmt<F: FnOnce(&Expr) -> ()>(stmt: &Stmt, assert: F) {
        match stmt {
            Stmt::Expr { expr } => assert(expr),
            _ => panic!("not an expression statement"),
        };
    }

    fn assert_block_stmt(stmt: &Stmt, assert: AssertExpr) {
        match stmt {
            Stmt::Block { token, stmts } => stmts.iter().for_each(|s| assert_expr_stmt(s, assert)),
            _ => panic!("not an expression statement"),
        };
    }

    fn assert_infix_expr(expr: &Expr, eop: &[u8], eleft: AssertExpr, eright: AssertExpr) {
        if let Expr::Infix {
            op, left, right, ..
        } = expr
        {
            assert_eq!(op, &eop);
            eleft(left);
            eright(right);
        } else {
            panic!("not an infix expression")
        }
    }

    #[test]
    fn test_let_stmts() {
        let input = "let x = 5;
let y = 10;
";
        assert_prog(input, |stmts| {
            assert_let_stmt(&stmts[0], b"x", |e| assert_int_literal(e, 5));
            assert_let_stmt(&stmts[1], b"y", |e| assert_int_literal(e, 10));
        })
    }

    #[test]
    fn test_return_stmts() {
        let input = "return 10;
return 2 * 3;
";
        assert_prog(input, |stmts| {
            dbg!(stmts);
            assert_return_stmt(&stmts[0], |e| assert_int_literal(e, 10));
            assert_return_stmt(&stmts[1], |e| {
                assert_infix_expr(
                    e,
                    b"*",
                    |e| assert_int_literal(e, 2),
                    |e| assert_int_literal(e, 3),
                )
            });
        })
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
            value: Expr::Identifier {
                token: Token::new(TokenType::IDENT, b"y"),
                value: b"y",
            },

        };
        eprintln!("{a}");
        assert_eq!(format!("{a}"), "let x = y;");
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

    #[test]
    fn test_prefix_expr() {
        let inputs = [("!5", b"!", 5 as i64), ("-10", b"-", 10 as i64)];
        for (input, eop, eexpr) in inputs {
            assert_prog(input, |stmts| {
                assert_eq!(stmts.len(), 1);
                if let Some(Stmt::Expr { expr }) = stmts.first() {
                    if let Expr::Prefix { op, expr, .. } = expr {
                        assert_eq!(op, eop);
                        assert!(matches!(**expr, Expr::IntLiteral {value,..} if value == eexpr));
                    } else {
                        eprintln!("expr doesn't match");
                        panic!()
                    }
                }
            });
        }
    }

    #[test]
    fn test_infix_expr() {
        let inputs = [
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];
        for (input, eleft, eop, eright) in inputs {
            assert_prog(input, |stmts| {
                if let Some(Stmt::Expr { expr }) = stmts.first() {
                    if let Expr::Infix {
                        op, left, right, ..
                    } = expr
                    {
                        assert_eq!(op, &eop.as_bytes());
                        assert_int_literal(left, eleft);
                        assert_int_literal(right, eright);
                    } else {
                        panic!()
                    }
                }
            });
        }
    }

    #[test]
    fn test_complex_expr() {
        let inputs = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("5 < 4 != 3 < 4", "((5 < 4) != (3 < 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];
        for (i, o) in inputs {
            assert_prog(i, |stmts| {
                assert_eq!(o, stmts[0].to_string());
            })
        }
    }

    #[test]
    fn test_bool_literal() {
        let inputs = [
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
        ];
        for (i, o) in inputs {
            assert_prog(i, |stmts| {
                assert_eq!(o, stmts[0].to_string());
            })
        }
    }

    #[test]
    fn test_group_expr() {
        let inputs = [
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];
        for (i, o) in inputs {
            assert_prog(i, |stmts| {
                assert_eq!(o, stmts[0].to_string());
            })
        }
    }

    #[test]
    fn test_if_expr() {
        let input = "if (x < y) { x }";
        assert_prog(input, |stmts| match &stmts[0] {
            Stmt::Expr { expr } => match expr {
                Expr::If {
                    token,
                    condition,
                    consequence,
                    alternative,
                } => {
                    assert_infix_expr(
                        &condition,
                        b"<",
                        |e| assert_ident(e, b"x"),
                        |e| assert_ident(e, b"y"),
                    );

                    assert_block_stmt(&consequence, |e| assert_ident(e, b"x"));
                    assert!(alternative.is_none());
                }
                _ => panic!("not an If expr, got {:?}", expr),
            },
            _ => panic!("not an expr"),
        })
    }

    #[test]
    fn test_if_else_expr() {
        let input = "if (x < y) { x } else { y }";
        assert_prog(input, |stmts| match &stmts[0] {
            Stmt::Expr { expr } => match expr {
                Expr::If {
                    token,
                    condition,
                    consequence,
                    alternative,
                } => {
                    assert_infix_expr(
                        &condition,
                        b"<",
                        |e| assert_ident(e, b"x"),
                        |e| assert_ident(e, b"y"),
                    );

                    assert_block_stmt(&consequence, |e| assert_ident(e, b"x"));
                    assert_block_stmt(alternative.as_deref().unwrap(), |e| assert_ident(e, b"y"));
                }
                _ => panic!("not an If expr, got {:?}", expr),
            },
            _ => panic!("not an expr"),
        })
    }

    #[test]
    fn test_fn_literal() {
        let input = "fn(x,y) { x + y; }";

        assert_prog(input, |stmts| {
            assert_expr_stmt(&stmts[0], |s| match s {
                Expr::FnLiteral {
                    token,
                    parameters,
                    block,
                } => {
                    assert_eq!(parameters.len(), 2);
                    assert_ident(&parameters[0], b"x");
                    assert_ident(&parameters[1], b"y");
                    match block.as_ref() {
                        Stmt::Block { token, stmts } => assert_expr_stmt(&stmts[0], |s| {
                            assert_infix_expr(
                                s,
                                b"+",
                                |e| assert_ident(e, b"x"),
                                |e| assert_ident(e, b"y"),
                            );
                        }),
                        _ => panic!("not a block statement"),
                    }
                }
                _ => panic!("not a fn literal"),
            });
        })
    }

    #[test]
    fn test_fn_params() {
        let inputs = [
            ("fn() {};", vec![]),
            ("fn(x) {};", vec![b"x"]),
            ("fn(x,y,z) {};", vec![b"x", b"y", b"z"]),
        ];
        for (input, eparams) in inputs {
            assert_prog(input, |stmts| {
                assert_expr_stmt(&stmts[0], |s| match s {
                    Expr::FnLiteral {
                        token,
                        parameters,
                        block,
                    } => {
                        for i in 0..eparams.len() {
                            assert_ident(&parameters[i], eparams[i]);
                        }
                    }
                    _ => panic!("not a fn literal"),
                })
            })
        }
    }

    #[test]
    fn test_fn_call() {
        let input = "add(1, 2 * 3, 4 + 5);";
        assert_prog(input, |stmts| {
            assert_expr_stmt(&stmts[0], |s| match s {
                Expr::Call {
                    token,
                    arguments,
                    function,
                } => {
                    assert_ident(function, b"add");
                    assert_int_literal(&arguments[0], 1);
                    assert_infix_expr(
                        &arguments[1],
                        b"*",
                        |e| assert_int_literal(e, 2),
                        |e| assert_int_literal(e, 3),
                    );
                    assert_infix_expr(
                        &arguments[2],
                        b"+",
                        |e| assert_int_literal(e, 4),
                        |e| assert_int_literal(e, 5),
                    );
                }
                _ => panic!("not a call expr"),
            })
        })
    }

    fn assert_prog<F: Fn(&[Stmt])>(input: &str, assertions: F) {
        let mut p = Parser::new(Lexer::new(input.as_bytes()));
        let prog = p.parse();
        if let Ok(p) = prog {
            assertions(&p.stmts);
        } else {
            log_errors(&p.errors);
            panic!();
        }
    }
}
