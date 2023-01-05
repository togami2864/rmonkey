use std::fmt;

use rmonkey_ast::operator::{Infix, Prefix};

#[derive(Debug)]
pub enum EvalErrorKind {
    TypeMismatch {
        op: Infix,
        left: String,
        right: String,
    },
    UnknownInfixOperator {
        op: Infix,
        left: String,
        right: String,
    },
    UnknownPrefixOperator {
        op: Prefix,
        right: String,
    },
    UncaughtRef {
        ident: String,
    },
}

impl fmt::Display for EvalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalErrorKind::TypeMismatch { op, left, right } => {
                write!(f, "type mismatch: {} {} {}", left, op, right)
            }
            EvalErrorKind::UnknownInfixOperator { op, left, right } => {
                write!(f, "unknown operator: {} {} {}", left, op, right)
            }
            EvalErrorKind::UnknownPrefixOperator { op, right } => {
                write!(f, "unknown prefix operator; {}{}", op, right)
            }
            EvalErrorKind::UncaughtRef { ident } => write!(f, "identifier not found: {}", ident),
        }
    }
}
