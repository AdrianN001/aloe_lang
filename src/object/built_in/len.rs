use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, integer::Integer};

pub fn len_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ))));
    }

    match &*args[0].borrow() {
        Object::String(str) => Rc::new(RefCell::new(Object::Int(Integer {
            value: str.value.len() as i64,
        }))),
        Object::Array(arr) => Rc::new(RefCell::new(Object::Int(Integer {
            value: arr.items.len() as i64,
        }))),
        _ => Rc::new(RefCell::new(Object::new_error(format!(
            "unexpected argument type for len(): got {}",
            &args[0].borrow().get_type()
        )))),
    }
}
