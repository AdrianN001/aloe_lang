use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, error::panic_type::PanicType, float_obj::FloatObj, integer::Integer,
    panic_obj::PanicObj, state::StateRef,
};

impl Object {
    pub fn evaluate_prefix(&self, operator: &str, state: StateRef) -> Result<ObjectRef, PanicObj> {
        match operator {
            "!" => self.evaluate_bang_operator_expression(state),
            "-" => self.evaluate_minus_prefix_operator_expression(state),
            _ => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!("unexpected prefix operator: '{}'", operator),
                state.clone(),
            )),
        }
    }

    fn evaluate_bang_operator_expression(&self, state: StateRef) -> Result<ObjectRef, PanicObj> {
        if *self == Object::TRUE_BOOL_OBJECT {
            return Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                false,
            ))));
        } else if *self == Object::FALSE_BOOL_OBJECT || *self == Object::NULL_OBJECT {
            return Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                true,
            ))));
        }
        Err(PanicObj::new(
            PanicType::OperatorIsNotSupported,
            format!(
                "unexpected expression ('{}') on the right side of the '!' operator ",
                self.inspect()
            ),
            state.clone(),
        ))
    }

    fn evaluate_minus_prefix_operator_expression(
        &self,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        match self {
            Object::Int(integer) => Ok(Rc::new(RefCell::new(Object::Int(Integer {
                value: -integer.value,
            })))),
            Object::FloatObj(float) => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: -float.val,
            })))),

            _ => Err(PanicObj::new(
                PanicType::OperatorIsNotSupported,
                format!(
                    "unexpected expression ('{}') on the right side of the '-' operator",
                    self.inspect()
                ),
                state.clone(),
            )),
        }
    }
}
