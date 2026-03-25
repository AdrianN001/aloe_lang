use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
};

pub fn error_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
    if args.len() != 1 {
        return Err(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 argument for err(), got: {}", args.len()),
            state,
        ));
    }

    let arg_borrow = args[0].borrow();

    let error_message = if let Object::String(str) = &*arg_borrow {
        str.value.clone()
    } else {
        return Err(PanicObj::new(
            PanicType::WrongArgumentType,
            format!("expected 'str' as argument for err()"),
            state,
        ));
    };

    Ok(new_objectref(Object::new_error(
        ErrorType::CustomError(),
        error_message,
        state,
    )))
}
