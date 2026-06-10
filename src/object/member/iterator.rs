use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::{error_type::ErrorType, panic_type::PanicType},
    iterator::Iterator,
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
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
        args: &[ObjectRef],
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match name {
            "has_next" => self.has_next(args, state),
            "next" => self.next(args, state),
            "collect" => self.collect(args, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for iterator: '{}'", name),
                state,
            )),
        }
    }

    // Methods

    fn has_next(&self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "iterator.has_next() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        Ok(self._has_next())
    }

    fn next(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "iterator.next() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let next_object = self._next();
        if let Some(val) = next_object {
            return Ok(val);
        }

        Ok(Rc::new(RefCell::new(Object::new_error(
            ErrorType::IteratorRanOut,
            "iterator ran out.".into(),
            state,
        ))))
    }

    fn collect(&mut self, args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
        if !args.is_empty() {
            return Err(PanicObj::new(
                PanicType::WrongArgumentCount,
                format!(
                    "iterator.collect() takes no arguments, but {} were provided",
                    args.len()
                ),
                state,
            ));
        }
        let mut arr = Vec::new();

        while let Some(val) = self._next() {
            arr.push(val);
        }

        Ok(Rc::new(RefCell::new(Object::Array(Box::new(Array {
            items: arr,
        })))))
    }
}
