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
    BlockStmt { stmts: Vec<Stmt> },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::LetStmt { name, value } => write!(f, "let {} = {}", name, value),
            Stmt::ReturnStmt(value) => write!(f, "return {}", value),
            Stmt::ExprStmt(expr) => write!(f, "{}", expr),
            Stmt::BlockStmt { stmts } => {
                let stmts: Vec<String> = stmts.iter().map(|stmt| stmt.to_string()).collect();
                write!(f, "{}", stmts.join("\n"))
            }
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    IntLiteral(u64),
    BoolLiteral(bool),
    If {
        condition: Box<Expr>,
        consequence: Box<Stmt>,
        alternative: Option<Box<Stmt>>,
    },
    PrefixExpr {
        op: Prefix,
        right: Box<Expr>,
    },
    InfixExpr {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Infix,
    },
    Func {
        params: Option<Vec<Expr>>,
        body: Box<Stmt>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(val) => write!(f, "{}", val),
            Expr::IntLiteral(val) => write!(f, "{}", val),
            Expr::BoolLiteral(val) => write!(f, "{}", val),
            Expr::If {
                condition,
                consequence,
                alternative,
            } => match alternative {
                Some(alt) => {
                    write!(f, "if({}){{{}}}else{{{}}}", condition, consequence, alt)
                }
                None => {
                    write!(f, "if({}){{{}}}", condition, consequence)
                }
            },
            Expr::PrefixExpr { op, right } => write!(f, "({}{})", op, right),
            Expr::InfixExpr { left, right, op } => write!(f, "({} {} {})", left, op, right),
            Expr::Func { params, body } => match params {
                Some(params) => {
                    if params.len() == 1 {
                        write!(f, "fn({}){{{}}}", params[0], body)
                    } else {
                        let params: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                        write!(
                            f,
                            "fn({}){{{}}}",
                            params.join(", ").trim_end_matches(", "),
                            body
                        )
                    }
                }
                None => {
                    write!(f, "fn(){{{}}}", body)
                }
            },
        }
    }
}
