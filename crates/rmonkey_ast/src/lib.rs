use std::fmt;

use operator::{Infix, Prefix};
use serde::Serialize;

pub mod operator;
pub mod precedence;
#[derive(Debug, Serialize)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Program { stmts }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum Stmt {
    LetStmt { name: Expr, value: Expr },
    ReturnStmt(Expr),
    ExprStmt(Expr),
    BlockStmt { stmts: Vec<Stmt> },
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::LetStmt { name, value } => write!(f, "let {name} = {value}"),
            Stmt::ReturnStmt(value) => write!(f, "return {value}"),
            Stmt::ExprStmt(expr) => write!(f, "{expr}"),
            Stmt::BlockStmt { stmts } => {
                let stmts: Vec<String> = stmts.iter().map(|stmt| stmt.to_string()).collect();
                write!(f, "{}", stmts.join("\n"))
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum Expr {
    Ident(String),
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
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
        params: Vec<Expr>,
        body: Box<Stmt>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Array {
        elements: Vec<Expr>,
    },
    IndexExpr {
        left: Box<Expr>,
        index: Box<Expr>,
    },
    HashLiteral {
        pairs: Vec<(Expr, Expr)>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(val) => write!(f, "{val}"),
            Expr::IntLiteral(val) => write!(f, "{val}"),
            Expr::BoolLiteral(val) => write!(f, "{val}"),
            Expr::StringLiteral(val) => write!(f, "\"{val}\""),
            Expr::If {
                condition,
                consequence,
                alternative,
            } => match alternative {
                Some(alt) => {
                    write!(f, "if({condition}){{{consequence}}}else{{{alt}}}")
                }
                None => {
                    write!(f, "if({condition}){{{consequence}}}")
                }
            },
            Expr::PrefixExpr { op, right } => write!(f, "({op}{right})"),
            Expr::InfixExpr { left, right, op } => write!(f, "({left} {op} {right})"),
            Expr::Func { params, body } => {
                if params.is_empty() {
                    return write!(f, "fn(){{{body}}}");
                }
                let params: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                write!(
                    f,
                    "fn({}){{{}}}",
                    params.join(", ").trim_end_matches(", "),
                    body
                )
            }
            Expr::Call { callee, args } => {
                if args.is_empty() {
                    return write!(f, "{callee}()");
                }
                let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", callee, args.join(", ").trim_end_matches(", "))
            }
            Expr::Array { elements } => {
                let elems: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elems.join(", ").trim_end_matches(", "))
            }
            Expr::IndexExpr { left, index } => write!(f, "({left}[{index}])"),
            Expr::HashLiteral { pairs } => {
                let mut s: Vec<String> = Vec::new();
                for (key, val) in pairs.iter() {
                    s.push(format!("{key}: {val}"));
                }
                write!(f, "{{{}}}", s.join(", "))
            }
        }
    }
}
