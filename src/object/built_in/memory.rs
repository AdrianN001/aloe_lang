use std::rc::Rc;

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    integer::Integer,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub fn id_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            "id expects exactly 1 argument".into(),
            state.clone(),
        )));
    }

    let arg = &args[0];
    let id = Rc::as_ptr(arg) as usize;

    Ok(new_objectref(Object::Int(Integer { value: id as i64 })))
}

pub fn number_of_references_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            "number_of_references expects exactly 1 argument".into(),
            state.clone(),
        )));
    }

    let arg = &args[0];
    let strong_count = Rc::strong_count(arg);
    let weak_count = Rc::weak_count(arg);

    Ok(new_objectref(Object::Int(Integer {
        value: (strong_count + weak_count) as i64,
    })))
}

pub fn size_of_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            "size_of expects exactly 1 argument".into(),
            state.clone(),
        )));
    }

    let arg = &args[0];
    let size = std::mem::size_of_val(&*arg.borrow());

    Ok(new_objectref(Object::Int(Integer { value: size as i64 })))
}
