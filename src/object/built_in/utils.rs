use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    integer::Integer,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
    string_obj::StringObj,
};

pub fn type_builtin_function(
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

    Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
        value: (*args[0].borrow().get_type()).into(),
    })))))
}

pub fn inspect_builtin_function(
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

    Ok(Rc::new(RefCell::new(Object::String(Box::new(StringObj {
        value: (*args[0].borrow().inspect()).into(),
    })))))
}

pub fn line_number_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if !args.is_empty() {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 0 arguments, got {}", args.len()),
            state,
        )));
    }

    let line_number = state.borrow().current_line;

    Ok(new_objectref(Object::Int(Integer {
        value: line_number as i64,
    })))
}
