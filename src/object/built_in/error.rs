use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

pub fn error_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 argument for err(), got: {}", args.len()),
            state,
        )));
    }

    let arg_borrow = args[0].borrow();

    let error_message = if let Object::String(str) = &*arg_borrow {
        str.value.clone()
    } else {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!("expected 'str' as argument for err()"),
            state,
        )));
    };

    Ok(new_objectref(Object::new_error(
        ErrorType::CustomError(),
        error_message,
        state,
    )))
}

pub fn panic_buitin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 2 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 2 arguments for panic(), got: {}", args.len()),
            state,
        )));
    }

    let type_arg_borrow = args[0].borrow();
    let panic_type_str = if let Object::String(string) = &*type_arg_borrow {
        string.value.clone()
    } else {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!("expected 'str' as first argument for panic()"),
            state,
        )));
    };

    let msg_arg_borrow = args[1].borrow();

    let panic_message = if let Object::String(str) = &*msg_arg_borrow {
        str.value.clone()
    } else {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!("expected 'str' as argument for panic()"),
            state,
        )));
    };

    let panic_type = match PanicType::from_str(&panic_type_str) {
        Some(panic_type) => panic_type,
        None => {
            return Ok(new_objectref(Object::new_error(
                ErrorType::UnknownPanicType,
                format!("Unknown panic type: {}", panic_type_str),
                state,
            )));
        }
    };

    Err(RuntimeSignal::Panic(PanicObj::new(
        panic_type,
        panic_message,
        state,
    )))
}
