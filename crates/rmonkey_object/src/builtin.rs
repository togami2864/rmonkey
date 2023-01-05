use crate::Object;
use rmonkey_error::{RMonkeyError, Result};

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
            _ => {
                return Err(RMonkeyError::Custom(format!(
                    "arg to `len` not supported, got {}",
                    obj.obj_type()
                )))
            }
        },
        None => todo!(),
    }
}
