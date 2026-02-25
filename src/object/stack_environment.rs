use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::ObjectRef;

pub type EnvRef = Rc<RefCell<StackEnvironment>>;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct StackEnvironment {
    pub map: HashMap<String, ObjectRef>,
    outer: Option<EnvRef>,
}

impl StackEnvironment {
    pub fn new() -> Self {
        StackEnvironment {
            map: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: &EnvRef) -> Self {
        StackEnvironment {
            map: HashMap::new(),
            outer: Some(outer.clone()),
        }
    }

    pub fn set(&mut self, identifier: &str, value: &ObjectRef) {
        self.map.insert(identifier.into(), value.clone());
    }

    pub fn get(&self, identifier: &str) -> Option<ObjectRef> {
        match self.map.get(identifier) {
            Some(val) => Some(val.clone()),
            None => {
                if let Some(outer_scope) = &self.outer {
                    return outer_scope.borrow().get(identifier);
                }
                None
            }
        }
    }
}
