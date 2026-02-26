use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef,
    iterator::{Iterator, range_based_iterator::RangeBasedIterator},
};

pub fn range_builtin_function(args: &[ObjectRef]) -> ObjectRef {
    match args.len() {
        // range(end)
        1 => {
            if let Object::Int(end) = &*args[0].borrow() {
                if end.value.is_positive() {
                    return Rc::new(RefCell::new(Object::Iterator(
                        Iterator::RangeBasedIterator(RangeBasedIterator::new(end.value)),
                    )));
                }

                return Rc::new(RefCell::new(Object::new_error(
                    "got a non-positve value as range input".into(),
                )));
            }

            Rc::new(RefCell::new(Object::new_error(
                "got a non-integer value as range input".into(),
            )))
        }

        // range(start, end)
        2 => {
            let start = match &*args[0].borrow() {
                Object::Int(start_value) => start_value.value,
                _ => {
                    return Rc::new(RefCell::new(Object::new_error(
                        "got a non-integer value as range start".into(),
                    )));
                }
            };

            let end = match &*args[1].borrow() {
                Object::Int(end_value) => end_value.value,
                _ => {
                    return Rc::new(RefCell::new(Object::new_error(
                        "got a non-integer value as range end".into(),
                    )));
                }
            };

            if start > end {
                return Rc::new(RefCell::new(Object::Iterator(
                    Iterator::RangeBasedIterator(RangeBasedIterator::new_with_explicit_step(
                        start, end, -1,
                    )),
                )));
            }
            Rc::new(RefCell::new(Object::Iterator(
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
                    return Rc::new(RefCell::new(Object::new_error(
                        "got a non-integer value as range start".into(),
                    )));
                }
            };

            let end = match &*args[1].borrow() {
                Object::Int(end_value) => end_value.value,
                _ => {
                    return Rc::new(RefCell::new(Object::new_error(
                        "got a non-integer value as range end".into(),
                    )));
                }
            };

            let step = match &*args[2].borrow() {
                Object::Int(step_value) => step_value.value,
                _ => {
                    return Rc::new(RefCell::new(Object::new_error(
                        "got a non-integer value as range step".into(),
                    )));
                }
            };

            Rc::new(RefCell::new(Object::Iterator(
                Iterator::RangeBasedIterator(RangeBasedIterator::new_with_explicit_step(
                    start, end, step,
                )),
            )))
        }

        _ => Rc::new(RefCell::new(Object::new_error(format!(
            "expected 1,2 or 3 value, got {} value.",
            args.len()
        )))),
    }
}
