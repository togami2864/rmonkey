use std::{collections::HashMap, fmt};

use builtin::len;
use rmonkey_ast::{Expr, Stmt};
use rmonkey_error::Result;
use scope::Scope;

mod builtin;
pub mod scope;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
    String(String),
    BuiltIn {
        func: fn(Vec<Object>) -> Result<Object>,
    },
    ReturnValue(Box<Object>),
    Func {
        params: Vec<Expr>,
        body: Stmt,
        scope: Scope,
    },
    Array {
        elements: Vec<Object>,
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
            Object::String(_) => "STRING",
            Object::Array { .. } => "ARRAY",
            Object::BuiltIn { .. } => "Builtin",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(val) => write!(f, "{}", val),
            Object::Bool(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
            Object::String(val) => write!(f, "\"{}\"", val),
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
            Object::BuiltIn { .. } => write!(f, "[builtin func]"),
            Object::Array { elements } => {
                if elements.is_empty() {
                    write!(f, "[]")
                } else {
                    let elems: Vec<String> = elements.iter().map(|p| p.to_string()).collect();
                    write!(f, "[{}]", elems.join(", ").trim_end_matches(", "),)
                }
            }
        }
    }
}

pub fn builtins() -> HashMap<&'static str, Object> {
    let mut builtin: HashMap<&'static str, Object> = HashMap::new();
    builtin.insert("len", Object::BuiltIn { func: len });
    builtin
}
