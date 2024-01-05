#![allow(unused, dead_code)]

use crate::token::Token;
use core::fmt;

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
    FnLiteral {
        token: Token<'a>,
        parameters: Vec<Expr<'a>>,
        block: Box<Stmt<'a>>,
    },
    Call {
        token: Token<'a>,
        function: Box<Expr<'a>>,
        arguments: Vec<Expr<'a>>,
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
    If {
        token: Token<'a>,
        condition: Box<Expr<'a>>,
        consequence: Box<Stmt<'a>>,
        alternative: Option<Box<Stmt<'a>>>,
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
            Expr::FnLiteral {
                token,
                parameters,
                block,
            } => {
                write!(
                    f,
                    "{} ({}) {}",
                    token,
                    parameters
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    block
                )
            }
            Expr::Call {
                token,
                arguments,
                function,
            } => {
                write!(
                    f,
                    "{}({})",
                    function,
                    arguments
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            }
            Expr::If {
                consequence,
                alternative,
                token,
                condition,
            } => {
                if let Some(alt) = alternative {
                    write!(f, "if {} {} else {})", condition, consequence, alt)
                } else {
                    write!(f, "if {} {})", condition, consequence)
                }
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

#[derive(Debug)]
pub enum Stmt<'a> {
    Let {
        token: Token<'a>,
        name: Expr<'a>,
        value: Expr<'a>,
    },
    Return {
        token: Token<'a>,
        value: Expr<'a>,
    },
    Expr {
        expr: Expr<'a>,
    },
    Block {
        token: Token<'a>,
        stmts: Vec<Stmt<'a>>,
    },
}

impl fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr { expr } => write!(f, "{}", expr),
            Stmt::Block { stmts, .. } => write!(
                f,
                "{}",
                stmts.iter().map(|s| s.to_string()).collect::<String>()
            ),
            Stmt::Return { token, value } => {
                write!(
                    f,
                    "{} {};",
                    std::str::from_utf8(token.literal).unwrap(),
                    value
                )
            }
            Stmt::Let { name, token, value } => write!(
                f,
                "{} {} = {};",
                std::str::from_utf8(token.literal).unwrap(),
                name,
                value
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
