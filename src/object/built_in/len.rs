use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    integer::Integer,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub fn len_builtin_function(
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
        Object::String(str) => Ok(Rc::new(RefCell::new(Object::Int(Integer {
            value: str.value.len() as i64,
        })))),
        Object::Array(arr) => Ok(Rc::new(RefCell::new(Object::Int(Integer {
            value: arr.items.len() as i64,
        })))),
        Object::HashMap(hashmap) => Ok(hashmap.get_length()),

        _ => Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "unexpected argument type for len(): got {}",
                &args[0].borrow().get_type()
            ),
            state,
        ))),
    }
}
