use std::time::{SystemTime, UNIX_EPOCH};

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    integer::Integer,
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub fn sleep(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 argument for __sleep(), got: {}", args.len()),
            state,
        )));
    }

    let arg_borrow = args[0].borrow();

    match &*arg_borrow {
        Object::Int(integer) => {
            std::thread::sleep_ms(integer.value as u32);
        }
        other_type => {
            return Err(RuntimeSignal::Panic(PanicObj::new(
                PanicType::WrongArgumentType,
                format!(
                    "expected 'int' as argument type for __sleep(), got: '{}'",
                    other_type.get_type()
                ),
                state,
            )));
        }
    }

    Ok(new_objectref(Object::NULL_OBJECT))
}

pub fn time_builtin_function() -> Result<ObjectRef, RuntimeSignal> {
    let start = SystemTime::now();

    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();

    let in_ms = since_epoch.as_millis() as i64;

    Ok(new_objectref(Object::Int(Integer { value: in_ms })))
}
