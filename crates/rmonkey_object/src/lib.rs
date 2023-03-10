use rmonkey_ast::{Expr, Stmt};
use rmonkey_error::Result;
use scope::Scope;
use std::hash::Hash;
use std::{collections::HashMap, fmt, hash::Hasher};

pub mod builtin;
pub mod scope;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Hash(HashMap<Object, Object>),
}

#[allow(clippy::derived_hash_with_manual_eq)]
impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Object::Int(ref i) => i.hash(state),
            Object::Bool(ref b) => b.hash(state),
            Object::String(ref s) => s.hash(state),
            _ => "".hash(state),
        }
    }
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
            Object::BuiltIn { .. } => "BUILTIN",
            Object::Hash(_) => "HASH",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(val) => write!(f, "{val}"),
            Object::Bool(val) => write!(f, "{val}"),
            Object::Null => write!(f, "null"),
            Object::String(val) => write!(f, "\"{val}\""),
            Object::ReturnValue(obj) => write!(f, "return {obj}"),
            Object::Func { params, body, .. } => {
                if params.is_empty() {
                    write!(f, "fn(){{{body}}}")
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
            Object::Hash(pair) => {
                let mut s: Vec<String> = Vec::new();
                for (key, val) in pair.iter() {
                    s.push(format!("{key}: {val}"));
                }
                write!(f, "{{{}}}", s.join(", "))
            }
        }
    }
}
