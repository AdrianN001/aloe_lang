use std::{cell::RefCell, char, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    boolean::Boolean,
    error::{error_type::ErrorType, panic_type::PanicType},
    float_obj::FloatObj,
    integer::Integer,
    new_objectref,
    panic_obj::PanicObj,
    stack_environment::EnvRef,
    state::StateRef,
    string_obj::StringObj,
};

impl Integer {
    pub fn apply_attribute(&self, name: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match name {
            "is_negative" => Ok(self.is_negative()),
            "is_positive" => Ok(self.is_positive()),

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
            "as_utf_char" => Ok(self.as_utf_char(state)),
            "clone" => Ok(self.deep_clone()),

            _ => Err(PanicObj::new(
                PanicType::UnknownMethod,
                format!("unknown method for int: '{}'", name),
                state,
            )),
        }
    }
}

impl Integer {
    // Attributes

    pub fn is_negative(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.value.is_negative(),
        }))
    }

    pub fn is_positive(&self) -> ObjectRef {
        new_objectref(Object::Bool(Boolean {
            value: self.value.is_positive(),
        }))
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

    pub fn as_utf_char(&self, state: StateRef) -> ObjectRef {
        let bytes_value: u32 = match self.value.try_into() {
            Ok(bytes) => bytes,
            Err(_) => {
                return new_objectref(Object::new_error(
                    ErrorType::UTFValueCasting,
                    format!("{} can not be converted to character.", self.value),
                    state,
                ));
            }
        };

        let mapped_char = match char::from_u32(bytes_value) {
            Some(mapped_char) => mapped_char,
            None => {
                return new_objectref(Object::new_error(
                    ErrorType::UTFValueCasting,
                    format!("{} can not be converted to character.", self.value),
                    state,
                ));
            }
        };

        let str = String::from(mapped_char);

        new_objectref(Object::String(StringObj { value: str }))
    }

    pub fn deep_clone(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer { value: self.value })))
    }
}
