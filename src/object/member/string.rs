use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, float_obj::FloatObj, integer::Integer, string_obj::StringObj,
};

impl StringObj {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match name {
            "length" => self.length(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for string: '{}'",
                name
            )))),
        }
    }
    pub fn apply_method(&mut self, name: &str, _args: &[ObjectRef]) -> ObjectRef {
        match name {
            "reversed" => self.reversed(),
            "chars" => self.chars(),
            "as_float" => self.as_float(),
            "as_int" => self.as_int(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for string: '{}'",
                name
            )))),
        }
    }

    // Attributes

    fn length(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.value.len() as i64,
        })))
    }

    // Methods

    fn reversed(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::String(StringObj {
            value: self.value.chars().rev().collect(),
        })))
    }

    fn chars(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Array {
            items: self
                .value
                .chars()
                .map(|character| {
                    Rc::new(RefCell::new(Object::String(StringObj {
                        value: character.to_string(),
                    })))
                })
                .collect(),
        })))
    }

    fn as_float(&self) -> ObjectRef {
        match self.value.parse::<f64>() {
            Ok(float_value) => Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: float_value,
            }))),
            Err(err) => Rc::new(RefCell::new(Object::new_error(err.to_string()))),
        }
    }

    fn as_int(&self) -> ObjectRef {
        match self.value.parse::<i64>() {
            Ok(int_value) => Rc::new(RefCell::new(Object::Int(Integer { value: int_value }))),
            Err(err) => Rc::new(RefCell::new(Object::new_error(err.to_string()))),
        }
    }
}
