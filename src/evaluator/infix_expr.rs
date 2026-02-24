use std::{cell::RefCell, rc::Rc};

use crate::object::{Object, ObjectRef, boolean::Boolean, integer::Integer, string_obj::StringObj};

impl Object {
    pub fn evaluate_infix_expression(
        &self,
        right: ObjectRef,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match (self, &*right.borrow()) {
            (Object::Int(left_int), Object::Int(right_int)) => {
                Self::eval_int_int_infix_expression(left_int, right_int, operator)
            }

            (Object::Bool(left_bool), Object::Bool(right_bool)) => {
                Self::eval_bool_bool_infix_expression(left_bool, right_bool, operator)
            }

            (Object::String(left_str), Object::String(right_str)) => {
                Self::eval_str_str_infix_expression(left_str, right_str, operator)
            }
            _ => Err("unexpected operand types".into()),
        }
    }

    fn eval_int_int_infix_expression(
        left: &Integer,
        right: &Integer,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => Ok(Rc::new(RefCell::new(Object::Int(Integer {
                value: left.value + right.value,
            })))),
            "-" => Ok(Rc::new(RefCell::new(Object::Int(Integer {
                value: left.value - right.value,
            })))),

            "*" => Ok(Rc::new(RefCell::new(Object::Int(Integer {
                value: left.value * right.value,
            })))),
            "/" => Ok(Rc::new(RefCell::new(Object::Int(Integer {
                value: left.value / right.value,
            })))),

            "<" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value < right.value,
            )))),
            ">" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value > right.value,
            )))),

            "==" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value == right.value,
            )))),
            "!=" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value != right.value,
            )))),

            _ => Err("unexpected operator.".into()),
        }
    }

    fn eval_bool_bool_infix_expression(
        left: &Boolean,
        right: &Boolean,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match operator {
            "==" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value == right.value,
            )))),
            "!=" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value != right.value,
            )))),

            _ => Err("unexpected operator".into()),
        }
    }

    fn eval_str_str_infix_expression(
        left: &StringObj,
        right: &StringObj,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => Ok(Rc::new(RefCell::new(Object::String(StringObj {
                value: left.value.clone() + &right.value,
            })))),

            "==" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value == right.value,
            )))),
            "!=" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value != right.value,
            )))),

            _ => Err("unexpected operator".into()),
        }
    }
}
