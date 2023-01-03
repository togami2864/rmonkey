use std::fmt;

use operator::{Infix, Prefix};

pub mod operator;
pub mod precedence;
#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Program { stmts }
    }
}

#[derive(Debug)]
pub enum Stmt {
    LetStmt { name: Expr, value: Expr },
    ReturnStmt(Expr),
    ExprStmt(Expr),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::LetStmt { name, value } => write!(f, "let {} = {}", name, value),
            Stmt::ReturnStmt(value) => write!(f, "return {}", value),
            Stmt::ExprStmt(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    IntLiteral(u64),
    PrefixExpr {
        op: Prefix,
        right: Box<Expr>,
    },
    InfixExpr {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Infix,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(val) => write!(f, "{}", val),
            Expr::IntLiteral(val) => write!(f, "{}", val),
            Expr::PrefixExpr { op, right } => write!(f, "({}{})", op, right),
            Expr::InfixExpr { left, right, op } => write!(f, "({} {} {})", left, op, right),
        }
    }
}
