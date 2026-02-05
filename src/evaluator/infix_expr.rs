use crate::object::{Object, boolean::Boolean, integer::Integer};



impl Object{


    pub fn evaluate_infix_expression(&self, right: &Object, operator: &str) -> Result<Object, String>{

        match (self, right){
            (Object::Int(left_int), Object::Int(right_int)) => {
                Self::eval_int_int_infix_expression(left_int, right_int, operator)
            }
            (Object::Bool(left_bool), Object::Bool(right_bool)) => {
                Self::eval_bool_bool_infix_expression(left_bool, right_bool, operator)
            }
            _ => Err("unexpected operand types".into())
        }
    }

    fn eval_int_int_infix_expression(left: &Integer, right: &Integer, operator: &str) -> Result<Object, String>{
        match operator{
            "+" => Ok(Object::Int(Integer { value: left.value + right.value })), 
            "-" => Ok(Object::Int(Integer { value: left.value - right.value })),
            
            "*" => Ok(Object::Int(Integer { value: left.value * right.value })),
            "/" => Ok(Object::Int(Integer { value: left.value / right.value })),

            "<" => Ok(Object::get_native_boolean_object(left.value < right.value)),
            ">" => Ok(Object::get_native_boolean_object(left.value > right.value)),
            
            "==" => Ok(Object::get_native_boolean_object(left.value == right.value)),
            "!=" => Ok(Object::get_native_boolean_object(left.value != right.value)),

            _ => Err("unexpected operator.".into())
        }
    }

    fn eval_bool_bool_infix_expression(left: &Boolean, right: &Boolean, operator: &str) -> Result<Object, String>{
        match operator{
            "==" => Ok(Object::get_native_boolean_object(left.value == right.value)),
            "!=" => Ok(Object::get_native_boolean_object(left.value != right.value)),

            _ => Err("unexpected operator".into())
        }
    }
}
