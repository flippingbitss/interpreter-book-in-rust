#![allow(unused, dead_code)]

use core::fmt;
use crate::token::Token;

// Remove heap allocation per node to single allocation per AST
// with an arena alloc
#[derive(Debug)]
pub enum Expr<'a> {
    Identifier {
        token: Token<'a>,
        value: &'a [u8],
    },
    IntLiteral {
        token: Token<'a>,
        value: i64,
    },
    BoolLiteral {
        token: Token<'a>,
        value: bool,
    },
    Prefix {
        token: Token<'a>,
        op: &'a [u8],
        expr: Box<Expr<'a>>,
    },
    Infix {
        token: Token<'a>,
        left: Box<Expr<'a>>,
        op: &'a [u8],
        right: Box<Expr<'a>>,
    },
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Identifier { token, .. } => {
                write!(f, "{}", token)
            }
            Expr::IntLiteral { token, .. } => {
                write!(f, "{}", token)
            }
            Expr::BoolLiteral { token, .. } => {
                write!(f, "{}", token)
            }
            Expr::Prefix { expr, op, .. } => {
                write!(f, "({}{})", std::str::from_utf8(op).unwrap(), expr)
            }
            Expr::Infix {
                left, right, op, ..
            } => {
                write!(
                    f,
                    "({} {} {})",
                    left,
                    std::str::from_utf8(op).unwrap(),
                    right
                )
            }
        }
    }
}

pub struct Program<'a> {
    pub stmts: Vec<Stmt<'a>>,
}

pub enum Stmt<'a> {
    Let { token: Token<'a>, name: Expr<'a> },
    Return { token: Token<'a> },
    Expr { expr: Expr<'a> },
}

impl fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => write!(f, "{}", expr),
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
