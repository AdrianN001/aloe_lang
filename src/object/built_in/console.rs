use std::{cell::RefCell, io::Write, rc::Rc};

use crate::object::{
    Object, ObjectRef, new_objectref, stack_environment::EnvRef, string_obj::StringObj,
};

// print(object, ends)
pub fn console_write_builtin_function(args: &[ObjectRef], _environ: EnvRef) -> ObjectRef {
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

// println(object_0, ...object_n)
pub fn console_writeln_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    args.iter().for_each(|arg| {
        let borrow = arg.borrow();
        print!("{} ", borrow.inspect());
    });
    println!();

    Rc::new(RefCell::new(Object::NULL_OBJECT))
}

// __input()
pub fn console_read_builtin_function() -> ObjectRef {
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();

    // \n
    buffer.pop();

    new_objectref(Object::String(StringObj { value: buffer }))
}
