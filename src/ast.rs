use core::fmt;

use crate::token::Token;

#[derive(Copy, Clone)]
pub struct Identifier<'a> {
    pub token: Token<'a>,
    pub value: &'a [u8],
}

#[derive(Copy, Clone)]
pub struct Expr<'a> {
    pub token: Token<'a>,
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.token.literal).unwrap())
    }
}

pub struct Program<'a> {
    pub stmts: Vec<Stmt<'a>>,
}

#[derive(Copy, Clone)]
pub enum Stmt<'a> {
    Let {
        token: Token<'a>,
        name: Identifier<'a>,
    },
    Return {
        token: Token<'a>,
    },
    Expr {
        expr: Expr<'a>,
    },
}

impl fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => {
                write!(f, "{};", std::str::from_utf8(expr.token.literal).unwrap())
            }
            Stmt::Return { token } => {
                write!(f, "{} <>;", std::str::from_utf8(token.literal).unwrap())
            }
            Stmt::Let { name, token } => write!(
                f,
                "{} {} = <expr>;",
                std::str::from_utf8(token.literal).unwrap(),
                std::str::from_utf8(name.token.literal).unwrap(),
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
