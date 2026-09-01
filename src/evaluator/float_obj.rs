use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expression::float_literal::FloatLiteral,
    object::{Object, ObjectRef, float::Float},
};

impl FloatLiteral {
    pub fn evaluate(&self) -> ObjectRef {
        // Parse the token literal directly as f64 to support scientific notation
        let float_value = self
            .token
            .literal
            .replace('_', "")
            .parse::<f64>()
            .unwrap_or_else(|_| {
                // Fallback to the old format if parsing fails
                let str = format!("{}.{}", self.integer_part, self.float_part);
                str.parse::<f64>().unwrap_or(0.0)
            });

        Rc::new(RefCell::new(Object::Float(Float { val: float_value })))
    }
}
