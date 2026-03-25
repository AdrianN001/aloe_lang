use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, float_obj::FloatObj, integer::Integer,
    panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef, string_obj::StringObj,
};

impl Integer {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for int: '{}'", name),
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
            "as_str" => Ok(self.as_str()),
            "as_float" => Ok(self.as_float()),
            "clone" => Ok(self.deep_clone()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for int: '{}'", name),
                state,
            )),
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
        Rc::new(RefCell::new(Object::Int(Integer { value: self.value })))
    }
}
