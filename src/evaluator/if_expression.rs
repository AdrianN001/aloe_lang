use std::{cell::RefCell, rc::Rc};

use crate::object::stack_environment::EnvRef;

use crate::{
    ast::expression::if_expression::IfExpression,
    object::{Object, ObjectRef, null::Null},
};

impl IfExpression {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let condition = self.condition.evaluate(environ.clone())?;

        if condition.borrow().is_truthy() {
            return self.consequence.evaluate(environ.clone()).clone();
        } else if let Some(alternative) = &self.alternative {
            return alternative.evaluate(environ.clone());
        }

        Ok(Rc::new(RefCell::new(Object::Null(Null {}))))
    }
}
