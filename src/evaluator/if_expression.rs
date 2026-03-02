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
            return self.consequence.evaluate(environ.clone());
        } else if !self.alternatives.is_empty(){

            for (alternative_cond_expr, alternative_block_smt) in &self.alternatives{
                let alternative_cond = alternative_cond_expr.evaluate(environ.clone())?;
                
                if alternative_cond.borrow().is_truthy(){
                    return alternative_block_smt.evaluate(environ.clone());
                }
            }

        } 

        if let Some(else_block) = &self.else_block {
            return else_block.evaluate(environ.clone());
        }

        Ok(Rc::new(RefCell::new(Object::Null(Null {}))))
    }
}
