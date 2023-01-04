use std::fmt;

pub mod scope;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
    ReturnValue(Box<Object>),
}

impl Object {
    pub fn obj_type(&self) -> &str {
        match self {
            Object::Int(_) => "INTEGER",
            Object::Bool(_) => "BOOLEAN",
            Object::Null => "NULL",
            Object::ReturnValue(_) => "RETURN_VALUE",
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
        }
    }
}
