use crate::object::{Object, ObjectRef, error::panic_type::PanicType, new_objectref, panic_obj::PanicObj, state::StateRef};


pub fn sleep(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj>{
    if args.len() != 1 {
        return Err(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 argument for __sleep(), got: {}", args.len()),
            state,
        ));
    }

    let arg_borrow = args[0].borrow();

    match &*arg_borrow{
        Object::Int(integer) => {
            std::thread::sleep_ms(integer.value as u32); 
        }
        other_type => return Err(PanicObj::new(
            PanicType::WrongArgumentType,
            format!("expected 'int' as argument type for __sleep(), got: '{}'", other_type.get_type()),
            state
        ))
    }

    Ok(new_objectref(Object::NULL_OBJECT))
}
