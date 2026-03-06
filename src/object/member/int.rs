use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, float_obj::FloatObj, integer::Integer, stack_environment::EnvRef, state::StateRef, string_obj::StringObj
};

impl Integer {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> ObjectRef {
        match name {
            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for int: '{}'",
                name
            ), state))),
        }
    }
    pub fn apply_method(&mut self, name: &str, _args: &[ObjectRef], environ: EnvRef, state: StateRef) -> ObjectRef {
        match name {
            "as_str" => self.as_str(),
            "as_float" => self.as_float(),
            "clone" => self.deep_clone(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for int: '{}'",
                name
            ), state))),
        }
    }

    // Methods

    pub fn as_str(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.value.to_string(),
        })))
    }

    pub fn as_float(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::FloatObj(FloatObj {
            val: self.value as f64,
        })))
    }

    pub fn deep_clone(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer{
            value: self.value
        })))
    }
}
