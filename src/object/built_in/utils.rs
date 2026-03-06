use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, stack_environment::EnvRef, state::StateRef, string_obj::StringObj};

pub fn type_builtin_function(args: &[ObjectRef], state: StateRef) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ), state)));
    }

    Rc::new(RefCell::new(Object::String(StringObj {
        value: (*args[0].borrow().get_type()).into(),
    })))
}

pub fn inspect_builtin_function(args: &[ObjectRef], state: StateRef) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ), state)));
    }

    Rc::new(RefCell::new(Object::String(StringObj {
        value: (*args[0].borrow().inspect()).into(),
    })))
}
