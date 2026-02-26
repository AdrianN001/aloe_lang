use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, string_obj::StringObj};

pub fn type_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ))));
    }

    Rc::new(RefCell::new(Object::String(StringObj {
        value: (*args[0].borrow().get_type()).into(),
    })))
}

pub fn inspect_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1 value, got {} value.",
            args.len()
        ))));
    }

    Rc::new(RefCell::new(Object::String(StringObj {
        value: (*args[0].borrow().inspect()).into(),
    })))
}
