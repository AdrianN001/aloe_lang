use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, float_obj::FloatObj, integer::Integer, iterator::{Iterator, list_based_iterator::ListBasedIterator}, string_obj::StringObj
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
    pub fn apply_method(&mut self, name: &str, args: &[ObjectRef]) -> ObjectRef {
        match name {
            "reversed" => self.reversed(),
            "chars" => self.chars(),
            "as_float" => self.as_float(),
            "as_int" => self.as_int(),
            "split" => self.split(args),

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

    pub fn build_char_iterator(&self) -> Iterator{
        Iterator::ListBasedIterator(ListBasedIterator{
            list: self.value.chars().map(|char| Rc::new(RefCell::new(Object::String(StringObj { value: char.to_string() })))).collect(),
            index: 0
        })
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

    fn split(&self, args: &[ObjectRef]) -> ObjectRef{
        let split_value = if args.is_empty(){
            return self.chars();
        }else{
            match &*args[0].borrow(){
                Object::String(str) => str.value.clone(),
                other_type => return Rc::new(RefCell::new(Object::new_error(format!("expected to be the first paramter a 'str', got: {}", other_type.get_type()))))
            }
        };

        if split_value.is_empty(){
            return self.chars();
        }

        Rc::new(RefCell::new(Object::Array( Array{ 
                items: self.value.split(&split_value).map(|sub_str: &str|{
                    Rc::new(RefCell::new(Object::String(StringObj { value: sub_str.to_string() }
                )))
            }).collect()
        })))
    }
}
