use std::fmt;

use rmonkey_ast::{Expr, Stmt};
use scope::Scope;

pub mod scope;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
    Func {
        params: Vec<Expr>,
        body: Stmt,
        scope: Scope,
    },
}

impl Object {
    pub fn obj_type(&self) -> &str {
        match self {
            Object::Int(_) => "INTEGER",
            Object::Bool(_) => "BOOLEAN",
            Object::Null => "NULL",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Func { .. } => "FUNCTION",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(val) => write!(f, "{}", val),
            Object::Bool(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "return {}", obj),
            Object::Func { params, body, .. } => {
                if params.is_empty() {
                    write!(f, "fn(){{{}}}", body)
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
        }
    }
}
