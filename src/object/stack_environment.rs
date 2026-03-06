use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::ObjectRef;

pub type EnvRef = Rc<RefCell<StackEnvironment>>;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct StackEnvironment {
    pub map:                HashMap<String, ObjectRef>,
    pub outer:              Option<EnvRef>,
    stack_layer_name:       String
}

impl StackEnvironment {
    pub fn new() -> Self {
        StackEnvironment {
            map: HashMap::new(),
            outer: None,
            stack_layer_name: "<global>".into()
        }
    }

    pub fn new_enclosed(outer: EnvRef, stack_layer_name: String) -> Self {
        StackEnvironment {
            map: HashMap::new(),
            outer: Some(outer.clone()),
            stack_layer_name
        }
    }

    pub fn try_to_assign(&mut self, identifier: &str, value: ObjectRef) -> bool {
        if self.map.contains_key(identifier) {
            self.map.insert(identifier.into(), value);
            return true;
        }

        if let Some(outer) = &self.outer {
            return outer.borrow_mut().try_to_assign(identifier, value);
        }

        false
    }

    pub fn set(&mut self, identifier: &str, value: ObjectRef) {
        if !self.try_to_assign(identifier, value.clone()) {
            self.set_to_lowest_level(identifier, value);
        }
    }

    pub fn set_to_lowest_level(&mut self, identifier: &str, value: ObjectRef) {
        self.map.insert(identifier.into(), value);
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
