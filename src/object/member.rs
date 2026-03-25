use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, error::panic_type::PanicType, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef};

pub mod array;
pub mod float;
pub mod hashmap;
pub mod int;
pub mod iterator;
pub mod string;

impl Object {
    pub fn apply_attribute(&self, name: &str, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        if let Some(result) = self.check_early_attributes(name) {
            return Ok(result);
        }

        match self {
            Object::String(str) => str.apply_attribute(name, state),
            Object::Array(arr) => arr.apply_attribute(name, environ, state),
            Object::Int(int) => int.apply_attribute(name, state),
            Object::FloatObj(float) => float.apply_attribute(name, state),
            Object::Iterator(iterator) => iterator.apply_attribute(name, state),
            Object::HashMap(hashmap) => hashmap.apply_attribute(name, environ, state),

            Object::StructObject(struct_obj) => struct_obj.apply_attribute(name, environ, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("{} has no attribute", self.get_type()),
                state,
            )),
        }
    }

    pub fn apply_method(
        &mut self,
        name: &str,
        args: &[ObjectRef],
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match self {
            Object::String(str) => str.apply_method(name, args, environ, state),
            Object::Array(arr) => arr.apply_method(name, args, environ, state),
            Object::Int(int) => int.apply_method(name, args, environ, state),
            Object::FloatObj(float) => float.apply_method(name, args, environ, state),
            Object::Iterator(iterator) => iterator.apply_method(name, args, environ, state),
            Object::HashMap(hashmap) => hashmap.apply_method(name, args, environ, state),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("{} has no methods", self.get_type()),
                state,
            )),
        }
    }

    fn check_early_attributes(&self, name: &str) -> Option<ObjectRef> {
        match name {
            "is_ok" => match &self {
                Object::Err(_) => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(
                    false,
                )))),
                _ => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(
                    true,
                )))),
            },
            "is_err" => match &self {
                Object::Err(_) => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(
                    true,
                )))),
                _ => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(
                    false,
                )))),
            },

            _ => None,
        }
    }
}
