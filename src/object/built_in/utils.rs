use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, error::panic_type::PanicType, panic_obj::PanicObj, state::StateRef, string_obj::StringObj};

pub fn type_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
    if args.len() != 1 {
        return Err(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 value, got {} value.", args.len()),
            state,
        ));
    }

    Ok(Rc::new(RefCell::new(Object::String(StringObj {
        value: (*args[0].borrow().get_type()).into(),
    }))))
}

pub fn inspect_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
    if args.len() != 1 {
        return Err(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 value, got {} value.", args.len()),
            state,
        ));
    }

   Ok( Rc::new(RefCell::new(Object::String(StringObj {
        value: (*args[0].borrow().inspect()).into(),
    }))))
}
