use crate::object::{ObjectRef, error::panic_type::PanicType, panic_obj::{PanicObj, RuntimeSignal}, state::StateRef};



pub fn assert_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1{
        return Err(RuntimeSignal::Panic(
            PanicObj::new(
                PanicType::WrongArgumentCount,
                format!("assert() takes exactly one argument ({} given)", args.len()),
                state
            )
        ))
    }

    let condition = &args[0];
    if condition.borrow().is_truthy() {
        Ok(condition.clone())
    } else {
        Err(RuntimeSignal::Panic(
            PanicObj::new(
                PanicType::Assertion,
                "assertion failed".to_string(),
                state
            )
        ))
    }
}