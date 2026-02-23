use std::collections::HashMap;

use crate::object::Object;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct StackEnvironment {
    pub map: HashMap<String, Object>,
    outer: Option<Box<StackEnvironment>>,
}

impl StackEnvironment {
    pub fn new() -> Self {
        StackEnvironment {
            map: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: &StackEnvironment) -> Self {
        StackEnvironment {
            map: HashMap::new(),
            outer: Some(Box::new(outer.clone())),
        }
    }

    pub fn set(&mut self, identifier: &str, value: Object) {
        self.map.insert(identifier.into(), value);
    }

    pub fn get(&self, identifier: &str) -> Option<&Object> {
        match self.map.get(identifier) {
            Some(val) => Some(val),
            None => {
                if let Some(outer_scope) = &self.outer {
                    return outer_scope.get(identifier);
                }
                None
            }
        }
    }

    pub fn get_owned(&self, identifier: &str) -> Option<Object> {
        match self.get(identifier) {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }
}
