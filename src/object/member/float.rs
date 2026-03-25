use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, float_obj::FloatObj, integer::Integer, panic_obj::PanicObj, stack_environment::EnvRef, state::StateRef, string_obj::StringObj
};

impl FloatObj {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            _ => Err(PanicObj::new(
                PanicType::UnknownAttribute,
                format!("unknown attribute for float: '{}'", name),
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
            "as_int" => Ok(self.as_int()),
            "clone" => Ok(self.deep_copy()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for float: '{}'", name),
                state,
            )),
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
