use std::{cell::RefCell, rc::Rc};

use crate::object::{
    Object, ObjectRef, boolean::Boolean, float_obj::FloatObj, integer::Integer,
    string_obj::StringObj,
};

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

            (Object::Int(left_int), Object::FloatObj(right_float)) => {
                Self::eval_int_float_infix_expression(left_int, right_float, operator)
            }

            (Object::FloatObj(left_float), Object::Int(right_int)) => {
                Self::eval_float_int_infix_expression(left_float, right_int, operator)
            }

            (Object::FloatObj(left_float), Object::FloatObj(right_float)) => {
                Self::eval_float_float_infix_expression(left_float, right_float, operator)
            }

            (Object::Bool(left_bool), Object::Bool(right_bool)) => {
                Self::eval_bool_bool_infix_expression(left_bool, right_bool, operator)
            }

            (Object::String(left_str), Object::String(right_str)) => {
                Self::eval_str_str_infix_expression(left_str, right_str, operator)
            }

            (left_type, right_type) => Err(format!(
                "unexpected operand types: {} {} {}",
                left_type.get_type(),
                operator,
                right_type.get_type()
            )),
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

            _ => Err(format!("unexpected operator: '{}'", operator)),
        }
    }

    fn eval_int_float_infix_expression(
        left: &Integer,
        right: &FloatObj,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.value as f64 + right.val,
            })))),
            "-" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.value as f64 - right.val,
            })))),

            "*" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.value as f64 * right.val,
            })))),
            "/" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.value as f64 / right.val,
            })))),

            "<" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.value as f64) < right.val,
            )))),
            ">" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.value as f64 > right.val,
            )))),

            "==" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.value as f64).to_bits() == right.val.to_bits(),
            )))),
            "!=" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.value as f64).to_bits() != right.val.to_bits(),
            )))),

            _ => Err(format!("unexpected operator: '{}'", operator)),
        }
    }

    fn eval_float_int_infix_expression(
        left: &FloatObj,
        right: &Integer,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val + right.value as f64,
            })))),
            "-" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val - right.value as f64,
            })))),

            "*" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val * right.value as f64,
            })))),
            "/" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val / right.value as f64,
            })))),

            "<" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.val) < right.value as f64,
            )))),
            ">" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.val > right.value as f64,
            )))),

            "==" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.val).to_bits() == (right.value as f64).to_bits(),
            )))),
            "!=" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.val).to_bits() != (right.value as f64).to_bits(),
            )))),

            _ => Err(format!("unexpected operator: '{}'", operator)),
        }
    }

    fn eval_float_float_infix_expression(
        left: &FloatObj,
        right: &FloatObj,
        operator: &str,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val + right.val,
            })))),
            "-" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val - right.val,
            })))),

            "*" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val * right.val,
            })))),
            "/" => Ok(Rc::new(RefCell::new(Object::FloatObj(FloatObj {
                val: left.val / right.val,
            })))),

            "<" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.val < right.val,
            )))),
            ">" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                left.val > right.val,
            )))),

            "==" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.val).to_bits() == (right.val).to_bits(),
            )))),
            "!=" => Ok(Rc::new(RefCell::new(Object::get_native_boolean_object(
                (left.val).to_bits() != (right.val).to_bits(),
            )))),

            _ => Err(format!("unexpected operator: '{}'", operator)),
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

            _ => Err(format!("unexpected operator: '{}'", operator)),
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

            _ => Err(format!("unexpected operator: '{}'", operator)),
        }
    }
}
