use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, integer::Integer, null::Null, stack_environment::EnvRef,
    string_obj::StringObj,
};

impl Array {
    pub fn apply_attribute(&self, name: &str) -> ObjectRef {
        match name {
            "length" => self.length(),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown attribute for array: '{}'",
                name
            )))),
        }
    }
    pub fn apply_method(&mut self, name: &str, args: &[ObjectRef], environ: EnvRef) -> ObjectRef {
        match name {
            "reversed" => self.reversed(),
            "push" => self.push(args),
            "extend" => self.extend(args),
            "clear" => self.clear(),

            "map" => self.map(args),
            "filter" => self.filter(args),

            _ => Rc::new(RefCell::new(Object::new_error(format!(
                "unknown method for array: '{}'",
                name
            )))),
        }
    }

    // Attributes

    fn length(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Int(Integer {
            value: self.items.len() as i64,
        })))
    }

    // Methods

    fn reversed(&mut self) -> ObjectRef {
        Rc::new(RefCell::new(Object::Array(Array {
            items: self.items.iter().rev().map(|item| item.clone()).collect(),
        })))
    }

    fn push(&mut self, args: &[ObjectRef]) -> ObjectRef {
        if !args.is_empty() {
            args.iter().for_each(|argument| {
                self.items.push(argument.clone());
            });
        }
        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn extend(&mut self, args: &[ObjectRef]) -> ObjectRef {
        if !args.is_empty()
            && let Object::Array(other_arr) = &*args[0].borrow()
        {
            other_arr.items.iter().for_each(|item| {
                self.items.push(item.clone());
            })
        }

        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn clear(&mut self) -> ObjectRef {
        self.items.clear();

        Rc::new(RefCell::new(Object::Null(Null {})))
    }

    fn map(&mut self, args: &[ObjectRef]) -> ObjectRef {
        if args.is_empty() {
            return Rc::new(RefCell::new(Object::new_error(
                "no function was provided for map".into(),
            )));
        }

        if let Object::Func(function) = &*args[0].borrow() {
            let mut mapped_array_content = Vec::new();

            for item in self.items.clone() {
                let mapped_item = function.apply([item.clone()].as_ref());
                match mapped_item {
                    Ok(ok_value) => mapped_array_content.push(ok_value.clone()),
                    Err(error) => return Rc::new(RefCell::new(Object::new_error(error))),
                }
            }

            return Rc::new(RefCell::new(Object::Array(Array {
                items: mapped_array_content,
            })));
        }

        Rc::new(RefCell::new(Object::new_error(
            "argument provided is not a function".into(),
        )))
    }

    fn filter(&mut self, args: &[ObjectRef]) -> ObjectRef {
        if args.is_empty() {
            return Rc::new(RefCell::new(Object::new_error(
                "no function was provided for filter".into(),
            )));
        }

        if let Object::Func(function) = &*args[0].borrow() {
            let mut mapped_array_content = Vec::new();

            for item in self.items.clone() {
                let boolean_output = function.apply([item.clone()].as_ref());
                match boolean_output {
                    Ok(ok_value) => {
                        if ok_value.borrow().is_truthy() {
                            mapped_array_content.push(item.clone())
                        }
                    }
                    Err(error) => return Rc::new(RefCell::new(Object::new_error(error))),
                }
            }

            return Rc::new(RefCell::new(Object::Array(Array {
                items: mapped_array_content,
            })));
        }

        Rc::new(RefCell::new(Object::new_error(
            "argument provided is not a function".into(),
        )))
    }
}
