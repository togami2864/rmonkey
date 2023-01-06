use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Object;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Scope {
    pub store: HashMap<String, Object>,
    pub parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new() -> Self {
        let store: HashMap<String, Object> = HashMap::new();
        Scope {
            store,
            parent: None,
        }
    }

    /// create child scope that has given scope as the parent scope
    pub fn new_enclosed_environment(parent_scope: Rc<RefCell<Scope>>) -> Self {
        Scope {
            store: Default::default(),
            parent: Some(parent_scope),
        }
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<Object> {
        match self.store.get(&key) {
            Some(val) => Some(val.clone()),
            None => {
                if let Some(parent) = &self.parent {
                    return parent.borrow().store.get(&key).cloned();
                }
                None
            }
        }
    }
}
