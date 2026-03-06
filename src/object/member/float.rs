use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, float_obj::FloatObj, integer::Integer, stack_environment::EnvRef,
    state::StateRef, string_obj::StringObj,
};

impl FloatObj {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> ObjectRef {
        match name {
            _ => Rc::new(RefCell::new(Object::new_error(
                format!("unknown attribute for float: '{}'", name),
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
            "as_str" => self.as_str(),
            "as_int" => self.as_int(),
            "clone" => self.deep_copy(),

            _ => Rc::new(RefCell::new(Object::new_error(
                format!("unknown method for float: '{}'", name),
                state,
            ))),
        }
    }

    // Methods

    pub fn as_str(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.val.to_string(),
        })))
    }

    pub fn as_int(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.val as i64,
        })))
    }

    pub fn deep_copy(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::FloatObj(FloatObj { val: self.val })))
    }
}
