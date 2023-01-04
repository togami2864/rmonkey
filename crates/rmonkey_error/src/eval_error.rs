use std::fmt;

use rmonkey_ast::operator::{Infix, Prefix};
use rmonkey_object::Object;

#[derive(Debug)]
pub enum EvalErrorKind {
    TypeMismatch {
        op: Infix,
        left: Object,
        right: Object,
    },
    UnknownInfixOperator {
        op: Infix,
        left: Object,
        right: Object,
    },
    UnknownPrefixOperator {
        op: Prefix,
        right: Object,
    },
}

impl fmt::Display for EvalErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalErrorKind::TypeMismatch { op, left, right } => write!(
                f,
                "type mismatch: {} {} {}",
                left.obj_type(),
                op,
                right.obj_type()
            ),
            EvalErrorKind::UnknownInfixOperator { op, left, right } => write!(
                f,
                "unknown operator: {} {} {}",
                left.obj_type(),
                op,
                right.obj_type()
            ),
            EvalErrorKind::UnknownPrefixOperator { op, right } => {
                write!(f, "unknown prefix operator; {}{}", op, right.obj_type())
            }
        }
    }
}
