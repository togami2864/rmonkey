use std::fmt;

#[derive(Debug)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
}

impl Object {
    pub fn obj_type(&self) -> &str {
        match self {
            Object::Int(_) => "INTEGER",
            Object::Bool(_) => "BOOLEAN",
            Object::Null => "NULL",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(val) => write!(f, "{}", val),
            Object::Bool(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
        }
    }
}
