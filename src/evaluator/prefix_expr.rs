

use crate::object::{Object, integer::Integer};


impl Object{

    pub fn evaluate_prefix(&self, operator: &str) -> Result<Object, String>{
        match operator{
            "!" => self.evaluate_bang_operator_expression(),
            "-" => self.evaluate_minus_prefix_operator_expression(),
            _ => Err("unexpected prefix operator.".into())
        }
    }

    fn evaluate_bang_operator_expression(&self) -> Result<Object, String>{
        if *self == Object::TRUE_BOOL_OBJECT{
            return Ok(Object::get_native_boolean_object(false)); 

        }else if *self == Object::FALSE_BOOL_OBJECT || *self == Object::NULL_OBJECT{
            return Ok(Object::get_native_boolean_object(true));

        }
        Err("unexpected expression on the right side".into())
    }

    fn evaluate_minus_prefix_operator_expression(&self) -> Result<Object, String>{
        match self{
            Object::Int(integer) => Ok(Object::Int(Integer{
                value: -integer.value
            })),
            _ => Err("unexpected expression on the right side".into())
        }
    }
}
