use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, float_obj::FloatObj, integer::Integer, stack_environment::EnvRef,
    string_obj::StringObj,
};

impl Integer {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match name {
            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for int: '{}'",
                name
            )))),
        }
    }
    pub fn apply_method(&mut self, name: &str, _args: &[ObjectRef], _environ: EnvRef) -> ObjectRef {
        match name {
            "as_str" => self.as_str(),
            "as_float" => self.as_float(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for int: '{}'",
                name
            )))),
        }
    }

    // Methods

    fn as_str(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.value.to_string(),
        })))
    }

    fn as_float(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::FloatObj(FloatObj {
            val: self.value as f64,
        })))
    }
}
