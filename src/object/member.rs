use std::{cell::{Ref, RefCell}, rc::Rc};

use crate::object::{Object, ObjectRef, stack_environment::EnvRef};

pub mod array;
pub mod float;
pub mod int;
pub mod iterator;
pub mod string;

impl Object {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {

        if let Some(result) = self.check_early_attributes(name){
            return result;
        }

        match self {
            Object::String(str) => str.apply_attribute(name),
            Object::Array(arr) => arr.apply_attribute(name),
            Object::Int(int) => int.apply_attribute(name),
            Object::FloatObj(float) => float.apply_attribute(name),
            Object::Iterator(iterator) => iterator.apply_attribute(name),

            _ => Rc::new(RefCell::new(Object::new_error(
                "type has no attribute".into(),
            ))),
        }
    }

    pub fn apply_method(&mut self, name: &str, args: &[ObjectRef], environ: EnvRef) -> ObjectRef {
        match self {
            Object::String(str) => str.apply_method(name, args),
            Object::Array(arr) => arr.apply_method(name, args, environ),
            Object::Int(int) => int.apply_method(name, args, environ),
            Object::FloatObj(float) => float.apply_method(name, args, environ),
            Object::Iterator(iterator) => iterator.apply_method(name, args, environ),

            _ => Rc::new(RefCell::new(Object::new_error(
                "type has no methods".into(),
            ))),
        }
    }


    fn check_early_attributes(&self, name: &str) -> Option<ObjectRef>{
        match name{
            "is_ok" => match &self{
                Object::Err(_) => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(false)))),
                _ => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(true))))
            },
            "is_err" => match &self{
                Object::Err(_) => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(true)))),
                _ => Some(Rc::new(RefCell::new(Object::get_native_boolean_object(false))))
            },

            _ => None
        }
    }
}
