use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::statement::block_statement::BlockStatement;
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
use crate::object::{Object, ObjectRef};

impl BlockStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::ReturnVal(_ret_val) => {
                    if state.borrow().is_function_context() {
                        return Ok(result.clone());
                    } else {
                        return Err("cannot return from a non-function context".to_string());
                    }
                }
                Object::Err(_) => return Ok(result.clone()),
                _ => {}
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_function_context(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, String> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err("unexpected break keyword in non-loop context".into());
                }
                Object::Continue => {
                    return Err("unexpected continue keyword in non-loop context".into());
                }
                Object::ReturnVal(_ret_val) => return Ok(result.clone()),
                Object::Err(_) => return Ok(result.clone()),
                _ => {}
            }
        }

        Ok(result)
    }
}
