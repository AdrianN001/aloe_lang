use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, iterator::Iterator, stack_environment::EnvRef,
};

impl Iterator {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match name {
            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for iterator: '{}'",
                name
            )))),
        }
    }
    pub fn apply_method(&mut self, name: &str, _args: &[ObjectRef], _environ: EnvRef) -> ObjectRef {
        match name {
            "has_next" => self.has_next(),
            "next" => self.next(),
            "collect" => self.collect(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for iterator: '{}'",
                name
            )))),
        }
    }

    // Methods

    fn has_next(&self) -> ObjectRef {
        self._has_next()
    }

    fn next(&mut self) -> ObjectRef {
        let next_object = self._next();
        if let Some(val) = next_object {
            return val;
        }

        Rc::new(RefCell::new(Object::new_error(
            "IteratorError: iterator ran out.".into(),
        )))
    }

    fn collect(&mut self) -> ObjectRef {
        let mut arr = Vec::new();

        while let Some(val) = self._next() {
            arr.push(val);
        }

        Rc::new(RefCell::new(Object::Array(Array { items: arr })))
    }
}
