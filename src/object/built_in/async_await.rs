use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    error::panic_type::PanicType,
    future::{future_kind::FutureKind, future_state::FutureState},
    new_objectref,
    panic_obj::{PanicObj, RuntimeSignal},
    state::StateRef,
};

// __spawn()
pub fn spawn_builtin_function(
    args: &[ObjectRef],
    state: StateRef,
) -> Result<ObjectRef, RuntimeSignal> {
    if args.len() != 1 {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1 value, got {} value.", args.len()),
            state,
        )));
    }

    let arg_borrow = args[0].borrow();
    let mut state_borrow = state.borrow_mut();

    if let Object::Future(future) = &*arg_borrow {
        if let FutureState::Pending(future_kind) = &future.state {
            if let FutureKind::Value(task) = future_kind {
                state_borrow.add_to_scheduler(Rc::new(RefCell::new(task.clone())));
            }
        }
    } else {
        return Err(RuntimeSignal::Panic(PanicObj::new(
            PanicType::WrongArgumentType,
            format!(
                "expected <type 'future'> as parameter for __sleep, got: '{}'",
                arg_borrow.get_type()
            ),
            state.clone(),
        )));
    }

    Ok(new_objectref(Object::NULL_OBJECT))
}
