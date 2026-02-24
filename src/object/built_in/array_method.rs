use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, array::Array, string_obj::StringObj};

pub fn rest_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ))));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            let new_string_value = if str.value.len() > 1 {
                &str.value[1..]
            } else {
                ""
            };

            Rc::new(RefCell::new(Object::String(StringObj {
                value: new_string_value.into(),
            })))
        }
        Object::Array(arr) => {
            let new_array_value = if arr.items.len() > 1 {
                &arr.items[1..]
            } else {
                &[]
            };

            Rc::new(RefCell::new(Object::Array(Array {
                items: new_array_value.into(),
            })))
        }

        _ => Rc::new(RefCell::new(Object::new_error(format!(
            "unexpected argument type for len(): got {}",
            &args[0].borrow().get_type()
        )))),
    }
}

pub fn first_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ))));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            if !str.value.is_empty() {
                Rc::new(RefCell::new(Object::String(StringObj {
                    value: str.value.chars().next().unwrap().to_string(),
                })))
            } else {
                Rc::new(RefCell::new(Object::NULL_OBJECT))
            }
        }
        Object::Array(arr) => {
            if !arr.items.is_empty() {
                arr.items[0].clone()
            } else {
                Rc::new(RefCell::new(Object::NULL_OBJECT))
            }
        }

        _ => Rc::new(RefCell::new(Object::new_error(format!(
            "unexpected argument type for len(): got {}",
            &args[0].borrow().get_type()
        )))),
    }
}

pub fn last_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ))));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            if !str.value.is_empty() {
                Rc::new(RefCell::new(Object::String(StringObj {
                    value: str.value.chars().next_back().unwrap().to_string(),
                })))
            } else {
                Rc::new(RefCell::new(Object::NULL_OBJECT))
            }
        }
        Object::Array(arr) => {
            if !arr.items.is_empty() {
                arr.items[arr.items.len() - 1].clone()
            } else {
                Rc::new(RefCell::new(Object::NULL_OBJECT))
            }
        }

        _ => Rc::new(RefCell::new(Object::new_error(format!(
            "unexpected argument type for len(): got {}",
            &args[0].borrow().get_type()
        )))),
    }
}

pub fn push_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 2 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 2 value, got {} value.",
            args.len()
        ))));
    }

    if !matches!(&*args[0].borrow(), Object::Array(_) | Object::String(_)) {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected the first value to be array or string, got {}.",
            args[0].borrow().get_type()
        ))));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            if let Object::String(second_parameter_str) = &*args[1].borrow() {
                return Rc::new(RefCell::new(Object::String(StringObj {
                    value: str.value.clone() + &second_parameter_str.value,
                })));
            }
            Rc::new(RefCell::new(Object::new_error(
                "unmatching types: push(String, not String)".into(),
            )))
        }
        Object::Array(arr) => {
            if let Object::Array(second_parameter_array) = &*args[1].borrow() {
                return Rc::new(RefCell::new(Object::Array(Array {
                    items: [&arr.items[..], &second_parameter_array.items[..]].concat(),
                })));
            }
            Rc::new(RefCell::new(Object::Array(Array {
                items: {
                    let mut new_array = arr.items.clone();
                    new_array.push(args[1].clone());
                    new_array
                },
            })))
        }

        _ => Rc::new(RefCell::new(Object::new_error(format!(
            "unexpected argument type for len(): got {}",
            &args[0].borrow().get_type()
        )))),
    }
}
