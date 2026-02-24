use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef};

// print(object, ends)
pub fn console_write_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    if args.is_empty() {
        return Rc::new(RefCell::new(Object::NULL_OBJECT));
    }

    print!("{}", args[0].borrow().inspect());

    if args.len() > 1
        && let Object::String(end_str) = &*args[1].borrow()
    {
        print!("{}", end_str.value);
    } else {
        println!();
    }

    Rc::new(RefCell::new(Object::NULL_OBJECT))
}
