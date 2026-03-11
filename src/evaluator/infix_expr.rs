use std::panic;

use crate::{
    ast::expression::{Expression, infix::InfixExpression},
    object::{
        Object, ObjectRef, array::Array, boolean::Boolean, float_obj::FloatObj, hashmap::HashMap,
        integer::Integer, stack_environment::EnvRef, state::StateRef, string_obj::StringObj,
    },
};

impl InfixExpression {
    pub fn evaluate_infix_expression(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        let left_side = self.left.evaluate(environ.clone(), state.clone())?;

        if self.operator == "??" {
            return Self::evaluate_coalescing(left_side, &self.right, environ, state);
        }

        let right_side = self.right.evaluate(environ.clone(), state.clone())?;

        match self.operator.as_str() {
            "+" | "-" | "*" | "/" | "**" | "%" | "==" | "!=" | "<" | ">" | "<<" | ">>" | "&"
            | "|" | "^" | "<=" | ">=" => match &*left_side.borrow() {
                Object::Int(integer) => Self::integer_infix_operation_dispatch(
                    integer,
                    &self.operator,
                    right_side,
                    state,
                ),
                Object::FloatObj(float) => {
                    Self::float_infix_operation_dispatch(float, &self.operator, right_side, state)
                }
                Object::String(string) => {
                    Self::string_infix_operation_dispatch(string, &self.operator, right_side, state)
                }
                Object::Array(array) => {
                    Self::array_infix_operation_dispatch(array, &self.operator, right_side, state)
                }
                Object::HashMap(hashmap) => Self::hashmap_infix_operation_dispatch(
                    hashmap,
                    &self.operator,
                    right_side,
                    state,
                ),
                Object::Bool(bool) => {
                    Self::boolean_infix_operation_dispatch(bool, &self.operator, right_side, state)
                }
                _ => Err(format!(
                    "unexpected operand types: {} {} {}",
                    left_side.borrow().get_type(),
                    self.operator,
                    right_side.borrow().get_type()
                )),
            },
            "&&" | "||" => {
                let (left_bool_side, right_bool_side) =
                    Self::convert_operands_to_bool(left_side, right_side)?;

                if let Object::Bool(bool_operand) = &*left_bool_side.borrow() {
                    Self::boolean_infix_operation_dispatch(
                        bool_operand,
                        &self.operator,
                        right_bool_side,
                        state,
                    )
                } else {
                    panic!()
                }
            }
            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                left_side.borrow().get_type(),
                other_operator,
                right_side.borrow().get_type()
            )),
        }
    }

    fn evaluate_coalescing(
        left: ObjectRef,
        right: &Expression,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        let left_bool = {
            let left_borrow = left.borrow();

            left_borrow.is_truthy()
        };

        if left_bool {
            Ok(left.clone())
        } else {
            right.evaluate(environ, state)
        }
    }

    fn integer_infix_operation_dispatch(
        integer: &Integer,
        operator: &str,
        right: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => integer.add(right, state),
            "-" => integer.sub(right, state),
            "*" => integer.mul(right, state),
            "/" => integer.div(right, state),
            "%" => integer.modulo(right, state),
            "**" => integer.power(right, state),
            "==" => integer.eq(right),
            "!=" => integer.neq(right),
            "<" => integer.lt(right, state),
            "<=" => integer.le(right, state),
            ">" => integer.gt(right, state),
            ">=" => integer.ge(right, state),
            "<<" => integer.lshift(right, state),
            ">>" => integer.rshift(right, state),
            "&" => integer.band(right, state),
            "|" => integer.bor(right, state),
            "^" => integer.bxor(right, state),

            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                "int",
                other_operator,
                right.borrow().get_type()
            )),
        }
    }

    fn float_infix_operation_dispatch(
        float: &FloatObj,
        operator: &str,
        right: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => float.add(right, state),
            "-" => float.sub(right, state),
            "*" => float.mul(right, state),
            "/" => float.div(right, state),
            "%" => float.modulo(right, state),
            "**" => float.power(right, state),
            "==" => float.eq(right),
            "!=" => float.neq(right),
            "<" => float.lt(right, state),
            "<=" => float.le(right, state),
            ">" => float.gt(right, state),
            ">=" => float.ge(right, state),

            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                "float",
                other_operator,
                right.borrow().get_type()
            )),
        }
    }

    fn string_infix_operation_dispatch(
        string: &StringObj,
        operator: &str,
        right: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => string.add(right, state),
            "-" => string.sub(right, state),
            "*" => string.mul(right, state),
            "/" => string.div(right, state),
            "%" => string.modulo(right, state),
            "**" => string.power(right, state),
            "==" => string.eq(right),
            "!=" => string.neq(right),
            "<" => string.lt(right, state),
            "<=" => string.le(right, state),
            ">" => string.gt(right, state),
            ">=" => string.ge(right, state),

            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                "string",
                other_operator,
                right.borrow().get_type()
            )),
        }
    }

    fn array_infix_operation_dispatch(
        array: &Array,
        operator: &str,
        right: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => array.add(right, state),
            "-" => array.sub(right, state),
            "*" => array.mul(right, state),
            "/" => array.div(right, state),
            "%" => array.modulo(right, state),
            "**" => array.power(right, state),
            "==" => array.eq(right),
            "!=" => array.neq(right),
            "<" => array.lt(right, state),
            "<=" => array.le(right, state),
            ">" => array.gt(right, state),
            ">=" => array.ge(right, state),

            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                "array",
                other_operator,
                right.borrow().get_type()
            )),
        }
    }

    fn hashmap_infix_operation_dispatch(
        hashmap: &HashMap,
        operator: &str,
        right: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => hashmap.add(right, state),
            "-" => hashmap.sub(right, state),
            "*" => hashmap.mul(right, state),
            "/" => hashmap.div(right, state),
            "%" => hashmap.modulo(right, state),
            "**" => hashmap.power(right, state),
            "==" => hashmap.eq(right),
            "!=" => hashmap.neq(right),
            "<" => hashmap.lt(right, state),
            "<=" => hashmap.le(right, state),
            ">" => hashmap.gt(right, state),
            ">=" => hashmap.ge(right, state),

            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                "hashmap",
                other_operator,
                right.borrow().get_type()
            )),
        }
    }

    fn boolean_infix_operation_dispatch(
        bool: &Boolean,
        operator: &str,
        right: ObjectRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        match operator {
            "+" => bool.add(right, state),
            "-" => bool.sub(right, state),
            "*" => bool.mul(right, state),
            "/" => bool.div(right, state),
            "%" => bool.modulo(right, state),
            "**" => bool.power(right, state),
            "==" => bool.eq(right),
            "!=" => bool.neq(right),
            "<" => bool.lt(right, state),
            "<=" => bool.le(right, state),
            ">" => bool.gt(right, state),
            ">=" => bool.ge(right, state),

            "&&" => bool.land(right, state),
            "||" => bool.lor(right, state),
            "^" => bool.lxor(right, state),

            "&" => bool.band(right, state),
            "|" => bool.bor(right, state),

            other_operator => Err(format!(
                "unexpected operand types: {} {} {}",
                "boolean",
                other_operator,
                right.borrow().get_type()
            )),
        }
    }

    fn convert_operands_to_bool(
        left: ObjectRef,
        right: ObjectRef,
    ) -> Result<(ObjectRef, ObjectRef), String> {
        let left_bool = match &*left.borrow() {
            Object::Int(int) => int.bool()?,
            Object::FloatObj(float) => float.bool()?,
            Object::Bool(_) => left.clone(),
            Object::Array(arr) => arr.bool()?,
            Object::String(str) => str.bool()?,
            Object::HashMap(hmap) => hmap.bool()?,
            Object::Iterator(hmap) => hmap.bool()?,
            other => return Err(format!("cannot cast {} to boolean.", other.get_type())),
        };

        let right_bool = match &*right.borrow() {
            Object::Int(int) => int.bool()?,
            Object::FloatObj(float) => float.bool()?,
            Object::Bool(_) => right.clone(),
            Object::Array(arr) => arr.bool()?,
            Object::String(str) => str.bool()?,
            Object::HashMap(hmap) => hmap.bool()?,
            Object::Iterator(hmap) => hmap.bool()?,
            other => return Err(format!("cannot cast {} to boolean.", other.get_type())),
        };

        Ok((left_bool, right_bool))
    }
}
