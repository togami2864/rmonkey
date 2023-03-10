use std::collections::HashMap;

use crate::Object;
use rmonkey_error::{RMonkeyError, Result};

pub fn builtins() -> HashMap<&'static str, Object> {
    let mut builtin: HashMap<&'static str, Object> = HashMap::new();
    builtin.insert("len", Object::BuiltIn { func: len });
    builtin.insert("first", Object::BuiltIn { func: first });
    builtin.insert("last", Object::BuiltIn { func: last });
    builtin.insert("rest", Object::BuiltIn { func: rest });
    builtin.insert("push", Object::BuiltIn { func: push });
    builtin.insert("puts", Object::BuiltIn { func: puts });
    builtin
}

pub fn len(args: Vec<Object>) -> Result<Object> {
    if args.len() > 1 {
        return Err(RMonkeyError::Custom(format!(
            "wrong number of args. got={}, want=1",
            args.len()
        )));
    }
    match args.get(0) {
        Some(obj) => match obj {
            Object::String(val) => Ok(Object::Int(val.len() as i64)),
            Object::Array { elements } => Ok(Object::Int(elements.len() as i64)),
            _ => {
                Err(RMonkeyError::Custom(format!(
                    "arg to `len` not supported, got {}",
                    obj.obj_type()
                )))
            }
        },
        None => Ok(Object::Null),
    }
}

pub fn first(args: Vec<Object>) -> Result<Object> {
    if args.len() > 1 {
        return Err(RMonkeyError::Custom(format!(
            "wrong number of args. got={}, want=1",
            args.len()
        )));
    }

    if let Some(Object::Array { elements }) = args.get(0) {
        match elements.get(0) {
            Some(ele) => return Ok(ele.clone()),
            None => return Err(RMonkeyError::Custom("elements is empty".to_string())),
        }
    }

    Ok(Object::Null)
}

pub fn last(args: Vec<Object>) -> Result<Object> {
    if args.len() > 1 {
        return Err(RMonkeyError::Custom(format!(
            "wrong number of args. got={}, want=1",
            args.len()
        )));
    } else if args.is_empty() {
        return Ok(Object::Null);
    }

    let obj = args.get(0).unwrap();

    if let Object::Array { elements } = obj {
        let last_index = elements.len() - 1;
        match elements.get(last_index) {
            Some(ele) => return Ok(ele.clone()),
            None => return Err(RMonkeyError::Custom("elements is empty".to_string())),
        }
    }

    Err(RMonkeyError::Custom(format!(
        "argument to `last` must be ARRAY, got {}",
        obj.obj_type()
    )))
}

pub fn rest(args: Vec<Object>) -> Result<Object> {
    if args.len() > 1 {
        return Err(RMonkeyError::Custom(format!(
            "wrong number of args. got={}, want=1",
            args.len()
        )));
    } else if args.is_empty() {
        return Ok(Object::Null);
    }

    let obj = args.get(0).unwrap();

    if let Object::Array { elements } = obj {
        let length = elements.len();
        if length > 0 {
            let new_array = &elements[1..length];
            return Ok(Object::Array {
                elements: new_array.to_vec(),
            });
        }
        return Ok(Object::Null);
    }

    Err(RMonkeyError::Custom(format!(
        "argument to `rest` must be ARRAY, got {}",
        obj.obj_type()
    )))
}

pub fn push(args: Vec<Object>) -> Result<Object> {
    if args.len() > 2 {
        return Err(RMonkeyError::Custom(format!(
            "wrong number of args. got={}, want=1",
            args.len()
        )));
    } else if args.is_empty() {
        return Ok(Object::Null);
    }

    let obj = args.get(0).unwrap();
    let value = args.get(1).unwrap().clone();

    if let Object::Array { elements } = obj {
        let mut new_array = elements.clone();
        new_array.push(value);
        return Ok(Object::Array {
            elements: new_array,
        });
    }

    Err(RMonkeyError::Custom(format!(
        "argument to `push` must be ARRAY, got {}",
        obj.obj_type()
    )))
}

fn puts(args: Vec<Object>) -> Result<Object> {
    for a in args.iter() {
        println!("{a}");
    }
    Ok(Object::Null)
}
