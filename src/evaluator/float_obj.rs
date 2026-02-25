use std::{cell::RefCell, rc::Rc};

use crate::{ast::expression::float_literal::FloatLiteral, object::{Object, ObjectRef, float_obj::FloatObj}};



impl FloatLiteral{
    pub fn evaluate(&self) -> ObjectRef{
        let str = format!("{}.{}", self.integer_part, self.float_part);
        
        Rc::new(RefCell::new(Object::FloatObj( FloatObj{
            val: str.parse::<f64>().unwrap()
        })))
    }
}
