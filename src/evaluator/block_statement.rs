use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::statement::block_statement::BlockStatement;
use crate::object::stack_environment::EnvRef;
use crate::object::{Object, ObjectRef};

impl BlockStatement {
    pub fn evaluate(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone())?;

            if let Object::ReturnVal(_) = &*result.borrow() {
                return Ok(result.clone());
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_function_context(&self, environ: EnvRef) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone())?;
            let borrowed_result = result.borrow();

            if let Object::BreakVal(_) = &*borrowed_result {
                return Err("unexpected break keyword in function context".into());
            } else if matches!(&*borrowed_result, Object::Continue) {
                return Err("unexpected continue keyword in function context".into());
            }

            if matches!(&*borrowed_result, Object::ReturnVal(_)) {
                return Ok(result.clone());
            }
        }

        Ok(result)
    }
}
