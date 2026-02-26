use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, integer::Integer};

#[derive(PartialEq, Eq, Clone)]
pub struct RangeBasedIterator {
    pub start: i64,
    pub end: i64,
    pub step: i64,

    current: i64,
}

impl RangeBasedIterator {
    pub fn new(end: i64) -> Self {
        Self {
            start: 0,
            end,
            step: 1,
            current: 0,
        }
    }

    pub fn new_with_explicit_start(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
            step: 1,
            current: start,
        }
    }
    pub fn new_with_explicit_step(start: i64, end: i64, step: i64) -> Self {
        Self {
            start,
            end,
            step,
            current: start,
        }
    }

    pub fn _has_next(&self) -> ObjectRef {
        Rc::new(RefCell::new(Object::get_native_boolean_object(
            if self.step.is_negative() {
                self.current > self.end
            } else {
                self.current < self.end
            },
        )))
    }

    pub fn _next(&mut self) -> Option<ObjectRef> {
        let ret_value = self.current;

        let is_minus_stepping = self.step.is_negative();

        if (is_minus_stepping && self.current <= self.end)
            || (!is_minus_stepping && self.current >= self.end)
        {
            return None;
        }

        self.current += self.step;

        Some(Rc::new(RefCell::new(Object::Int(Integer {
            value: ret_value,
        }))))
    }
}
