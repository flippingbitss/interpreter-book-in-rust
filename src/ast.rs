use core::fmt;

use crate::token::Token;

#[derive(Copy, Clone)]
pub enum Expr<'a> {
    Identifier { token: Token<'a>, value: &'a [u8] },
    IntLiteral { token: Token<'a>, value: i64 },
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Identifier { token, .. } => {
                write!(f, "{}", std::str::from_utf8(token.literal).unwrap())
            }
            Expr::IntLiteral { token, .. } => {
                write!(f, "{}", std::str::from_utf8(token.literal).unwrap())
            }
        }
    }
}

pub struct Program<'a> {
    pub stmts: Vec<Stmt<'a>>,
}

#[derive(Copy, Clone)]
pub enum Stmt<'a> {
    Let { token: Token<'a>, name: Expr<'a> },
    Return { token: Token<'a> },
    Expr { expr: Expr<'a> },
}

impl fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => write!(f, "{};", expr),
            Stmt::Return { token } => {
                write!(f, "{} <>;", std::str::from_utf8(token.literal).unwrap())
            }
            Stmt::Let { name, token } => write!(
                f,
                "{} {} = <expr>;",
                std::str::from_utf8(token.literal).unwrap(),
                name,
            ),
        }
    }
}

pub enum Node<'a> {
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Stmt(stmt) => format!("{stmt}"),
            Node::Expr(expr) => format!("{expr}"),
        };
        Ok(())
    }
}
