use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, float_obj::FloatObj, integer::Integer};

impl Object {
    pub fn evaluate_prefix(&self, operator: &str) -> Result<ObjectRef, String> {
        match operator {
            "!" => self.evaluate_bang_operator_expression(),
            "-" => self.evaluate_minus_prefix_operator_expression(),
            _ => Err("unexpected prefix operator.".into()),
        }
    }

    fn evaluate_bang_operator_expression(&self) -> Result<ObjectRef, String> {
        if *self == Object::TRUE_BOOL_OBJECT {
            return Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                false,
            ))));
        } else if *self == Object::FALSE_BOOL_OBJECT || *self == Object::NULL_OBJECT {
            return Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                true,
            ))));
        }
        Err("unexpected expression on the right side".into())
    }

    fn evaluate_minus_prefix_operator_expression(&self) -> Result<ObjectRef, String> {
        match self {
            Object::Int(integer) => Ok(Rc::new(RefCell::new(Object::Int(Integer {
                value: -integer.value,
            })))),
            Object::FloatObj(float) => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: -float.val,
            })))),
            _ => Err("unexpected expression on the right side".into()),
        }
    }
}
