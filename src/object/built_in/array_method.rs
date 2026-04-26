use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    array::Array,
    error::panic_type::PanicType,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
    string_obj::StringObj,
};

pub fn rest_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 value, got {} value.", args.len()),
            state,
        )));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            let new_string_value = if str.value.len() > 1 {
                &str.value[1..]
            } else {
                ""
            };

            Ok(Rc::new(RefCell::new(Object::String(StringObj {
                value: new_string_value.into(),
            }))))
        }
        Object::Array(arr) => {
            let new_array_value = if arr.items.len() > 1 {
                &arr.items[1..]
            } else {
                &[]
            };

            Ok(Rc::new(RefCell::new(Object::Array(Array {
                items: new_array_value.into(),
            }))))
        }

        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected argument type for rest(): got {}",
                &args[0].borrow().get_type()
            ),
            state,
        ))),
    }
}

pub fn first_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 value, got {} value.", args.len()),
            state,
        )));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            if !str.value.is_empty() {
                Ok(Rc::new(RefCell::new(Object::String(StringObj {
                    value: str.value.chars().next().unwrap().to_string(),
                }))))
            } else {
                Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
            }
        }
        Object::Array(arr) => {
            if !arr.items.is_empty() {
                Ok(arr.items[0].clone())
            } else {
                Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
            }
        }

        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected argument type for first(): got {}",
                &args[0].borrow().get_type()
            ),
            state,
        ))),
    }
}

pub fn last_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 value, got {} value.", args.len()),
            state,
        )));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            if !str.value.is_empty() {
                Ok(Rc::new(RefCell::new(Object::String(StringObj {
                    value: str.value.chars().next_back().unwrap().to_string(),
                }))))
            } else {
                Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
            }
        }
        Object::Array(arr) => {
            if !arr.items.is_empty() {
                Ok(arr.items[arr.items.len() - 1].clone())
            } else {
                Ok(Rc::new(RefCell::new(Object::NULL_OBJECT)))
            }
        }

        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected argument type for last(): got {}",
                &args[0].borrow().get_type()
            ),
            state,
        ))),
    }
}

pub fn push_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 2 value, got {} value.", args.len()),
            state,
        )));
    }

    if !matches!(&*args[0].borrow(), Object::Array(_) | Object::String(_)) {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "expected the first value to be array or string, got {}.",
                args[0].borrow().get_type()
            ),
            state,
        )));
    }

    match &*args[0].borrow() {
        Object::String(str) => {
            if let Object::String(second_parameter_str) = &*args[1].borrow() {
                return Ok(Rc::new(RefCell::new(Object::String(StringObj {
                    value: str.value.clone() + &second_parameter_str.value,
                }))));
            }
            Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                "unmatching types: push(String, not String)".into(),
                state,
            )))
        }
        Object::Array(arr) => {
            if let Object::Array(second_parameter_array) = &*args[1].borrow() {
                return Ok(Rc::new(RefCell::new(Object::Array(Array {
                    items: [&arr.items[..], &second_parameter_array.items[..]].concat(),
                }))));
            }
            Ok(Rc::new(RefCell::new(Object::Array(Array {
                items: {
                    let mut new_array = arr.items.clone();
                    new_array.push(args[1].clone());
                    new_array
                },
            }))))
        }

        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected argument type for push(): got {}",
                &args[0].borrow().get_type()
            ),
            state,
        ))),
    }
}
