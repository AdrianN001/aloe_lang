use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, boolean::Boolean, error::panic_type::PanicType, float_obj::FloatObj,
    integer::Integer, new_objectref, panic_obj::PanicObj, stack_environment::EnvRef,
    state::StateRef, string_obj::StringObj,
};

impl FloatObj {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "is_negative" => Ok(self.is_negative()),
            "is_positive" => Ok(self.is_positive()),

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
            "as_float" => Ok(self.as_float()),
            "clone" => Ok(self.deep_copy()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for float: '{}'", name),
                state,
            )),
        }
    }

    // Attributes

    pub fn is_negative(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.val.is_sign_negative(),
        }))
    }

    pub fn is_positive(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.val.is_sign_positive(),
        }))
    }

    // Methods

    pub fn as_str(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(Box::new(StringObj {
            value: self.val.to_string(),
        }))))
    }

    pub fn as_int(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.val as i64,
        })))
    }

    pub fn as_float(&self) -> ObjectRef {
        new_objectref(Object::FloatObj(FloatObj { val: self.val }))
    }

    pub fn deep_copy(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::FloatObj(FloatObj { val: self.val })))
    }
}
