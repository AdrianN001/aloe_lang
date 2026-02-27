use std::{cell::RefCell, clone, rc::Rc};

use crate::object::{
    Object, ObjectRef, array::Array, integer::Integer, iterator::{Iterator, list_based_iterator::ListBasedIterator}, null::Null, stack_environment::EnvRef, string_obj::StringObj
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
    pub fn apply_method(&mut self, name: &str, args: &[ObjectRef], _environ: EnvRef) -> ObjectRef {
        match name {
            "reversed" => self.reversed(),
            "push" => self.push(args),
            "extend" => self.extend(args),
            "clear" => self.clear(),

            "clone" => self.deep_copy(),

            "map" => self.map(args),
            "filter" => self.filter(args),

            "as_iter" => self.as_iter(),
            "join" => self.join(args),

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

    fn deep_copy(&self) -> ObjectRef{
        Rc::new(RefCell::new(Object::Array( Array{
            items: self.items.iter().map(|item|{
                Rc::new(RefCell::new((*item.borrow()).clone()))
            }).collect()
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

    fn as_iter(&self) -> ObjectRef{
        Rc::new(RefCell::new(Object::Iterator(
            self.build_iterator()
        )))
    }

    pub fn build_iterator(&self) -> Iterator{
        Iterator::ListBasedIterator(ListBasedIterator { 
            list: self.items.iter().map(|item| item.clone()).collect(), 
            index: 0
        })
    }

    fn join(&self, args: &[ObjectRef]) -> ObjectRef{
        let join_str_value = if args.is_empty() {
            "".to_string()
        } else { 
            match &*args[0].borrow(){
                Object::String(str) => str.value.clone(),
                other_type => return Rc::new(RefCell::new(Object::new_error(format!("expected to be the first paramter a 'str', got: {}", other_type.get_type()))))
            }
        };

        let mut strings = Vec::new();

        for (index, item) in self.items.clone().iter().enumerate(){
            match &*item.borrow(){
                Object::String(str) => strings.push(str.value.clone()),
                Object::Int(int) => strings.push(int.value.to_string()),
                Object::FloatObj(float) => strings.push(float.val.to_string()),
                Object::Bool(bool) => strings.push(bool.value.to_string()),

                other_type => return Rc::new(RefCell::new(Object::new_error(format!("not all elements can be converted to str. Element {} is {}", index, other_type.get_type()))))
            }
        }

        Rc::new(RefCell::new(Object::String(StringObj{
            value: strings.join(&join_str_value)
        })))

    }
}
