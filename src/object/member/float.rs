use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, float_obj::FloatObj, integer::Integer, stack_environment::EnvRef,
    string_obj::StringObj,
};

impl FloatObj {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match name {
            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for float: '{}'",
                name
            )))),
        }
    }
    pub fn apply_method(&mut self, name: &str, _args: &[ObjectRef], _environ: EnvRef) -> ObjectRef {
        match name {
            "as_str" => self.as_str(),
            "as_int" => self.as_int(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for float: '{}'",
                name
            )))),
        }
    }

    // Methods

    fn as_str(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.val.to_string(),
        })))
    }

    fn as_int(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.val as i64,
        })))
    }
}
