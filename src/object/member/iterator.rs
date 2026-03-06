use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    iterator::Iterator,
    stack_environment::EnvRef,
    state::StateRef,
};

impl Iterator {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> ObjectRef {
        match name {
            _ => Rc::new(RefCell::new(Object::new_error(
                format!("unknown attribute for iterator: '{}'", name),
                state,
            ))),
        }
    }
    pub fn apply_method(
        &mut self,
        name: &str,
        _args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> ObjectRef {
        match name {
            "has_next" => self.has_next(),
            "next" => self.next(state),
            "collect" => self.collect(),

            _ => Rc::new(RefCell::new(Object::new_error(
                format!("unknown method for iterator: '{}'", name),
                state,
            ))),
        }
    }

    // Methods

    fn has_next(&self) -> ObjectRef {
        self._has_next()
    }

    fn next(&mut self, state: StateRef) -> ObjectRef {
        let next_object = self._next();
        if let Some(val) = next_object {
            return val;
        }

        Rc::new(RefCell::new(Object::new_error(
            "IteratorError: iterator ran out.".into(),
            state,
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
