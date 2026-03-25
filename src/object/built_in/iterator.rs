use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    error::{error_type::ErrorType, panic_type::PanicType},
    iterator::{Iterator, range_based_iterator::RangeBasedIterator},
    new_objectref,
    panic_obj::PanicObj,
    state::StateRef,
};

pub fn range_builtin_function(args: &[ObjectRef], state: StateRef) -> Result<ObjectRef, PanicObj> {
    match args.len() {
        // range(end)
        1 => {
            if let Object::Int(end) = &*args[0].borrow() {
                if end.value.is_positive() {
                    return Ok(new_objectref(Object::Iterator(
                        Iterator::RangeBasedIterator(RangeBasedIterator::new(end.value)),
                    )));
                }

                return Ok(Rc::new(RefCell::new(Object::new_error(
                    ErrorType::RangeInput,
                    "got a non-positve value as range input".into(),
                    state,
                ))));
            }

            Err(PanicObj::new(
                PanicType::WrongArgumentType,
                "got a non-integer value as range input".into(),
                state,
            ))
        }

        // range(start, end)
        2 => {
            let start = match &*args[0].borrow() {
                Object::Int(start_value) => start_value.value,
                _ => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "got a non-integer value as range start".into(),
                        state,
                    ));
                }
            };

            let end = match &*args[1].borrow() {
                Object::Int(end_value) => end_value.value,
                _ => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "got a non-integer value as range end".into(),
                        state,
                    ));
                }
            };

            if start > end {
                return Ok(new_objectref(Object::Iterator(
                    Iterator::RangeBasedIterator(RangeBasedIterator::new_with_explicit_step(
                        start, end, -1,
                    )),
                )));
            }
            Ok(new_objectref(Object::Iterator(
                Iterator::RangeBasedIterator(RangeBasedIterator::new_with_explicit_start(
                    start, end,
                )),
            )))
        }

        // range(start, end, step)
        3 => {
            let start = match &*args[0].borrow() {
                Object::Int(start_value) => start_value.value,
                _ => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "got a non-integer value as range start".into(),
                        state,
                    ));
                }
            };

            let end = match &*args[1].borrow() {
                Object::Int(end_value) => end_value.value,
                _ => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "got a non-integer value as range end".into(),
                        state,
                    ));
                }
            };

            let step = match &*args[2].borrow() {
                Object::Int(step_value) => step_value.value,
                _ => {
                    return Err(PanicObj::new(
                        PanicType::WrongArgumentType,
                        "got a non-integer value as range step".into(),
                        state,
                    ));
                }
            };

            Ok(new_objectref(Object::Iterator(
                Iterator::RangeBasedIterator(RangeBasedIterator::new_with_explicit_step(
                    start, end, step,
                )),
            )))
        }

        _ => Err(PanicObj::new(
            PanicType::WrongArgumentCount,
            format!("expected 1,2 or 3 value, got {} value.", args.len()),
            state,
        )),
    }
}
