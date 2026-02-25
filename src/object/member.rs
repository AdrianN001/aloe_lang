use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef};

pub mod string;

impl Object {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match self {
            Object::String(str) => str.apply_attribute(name),
            _ => Rc::new(RefCell::new(Object::new_error(
                "type has no attribute".into(),
            ))),
        }
    }

    pub fn apply_method(&mut self, name: &str, args: &[ObjectRef]) -> ObjectRef {
        match self {
            Object::String(str) => str.apply_method(name, args),
            _ => Rc::new(RefCell::new(Object::new_error(
                "type has no methods".into(),
            ))),
        }
    }
}
