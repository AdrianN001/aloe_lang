use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, error::{error_type::ErrorType, panic_type::PanicType}, iterator::Iterator, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef
};

impl Iterator {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for iterator: '{}'", name),
                state,
            )),
        }
    }
    pub fn apply_method(
        &mut self,
        name: &str,
        _args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "has_next" => Ok(self.has_next()),
            "next" => Ok(self.next(state)),
            "collect" => Ok(self.collect()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for iterator: '{}'", name),
                state,
            )),
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
            ErrorType::IteratorRanOut,
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
