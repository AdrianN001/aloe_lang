use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, integer::Integer, string_obj::StringObj};

impl StringObj {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match name {
            "length" => self.length(),
            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for string: '{}'",
                name
            )))),
        }
    }
    pub fn apply_method(&mut self, name: &str, _args: &[ObjectRef]) -> ObjectRef {
        match name {
            "reversed" => self.reversed(),
            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for string: '{}'",
                name
            )))),
        }
    }

    // Attributes

    fn length(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.value.len() as i64,
        })))
    }

    // Methods

    fn reversed(&mut self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.value.chars().rev().collect(),
        })))
    }
}
