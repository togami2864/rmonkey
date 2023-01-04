use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Object;

#[derive(Debug, Default)]
pub struct Scope {
    store: Rc<RefCell<HashMap<String, Object>>>,
}

impl Scope {
    pub fn new() -> Self {
        let store: Rc<RefCell<HashMap<String, Object>>> = Rc::new(RefCell::new(HashMap::new()));
        Scope { store }
    }

    pub fn set(&mut self, key: String, val: Object) {
        self.store.borrow_mut().insert(key, val).unwrap();
    }

    pub fn get(&self, key: String) -> Option<Object> {
        self.store.borrow().get(&key).cloned()
    }
}
