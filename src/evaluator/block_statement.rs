use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::statement::block_statement::BlockStatement;
use crate::object::panic_obj::PanicObj;
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
use crate::object::{Object, ObjectRef};

impl BlockStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::ReturnVal(_ret_val) => {
                    if state.borrow().is_function_context() {
                        return Ok(result.clone());
                    } else {
                        return Err(PanicObj::new_simple(
                            "cannot return from a non-function context",
                            state.clone(),
                        ));
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
    ) -> Result<ObjectRef, PanicObj> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err(PanicObj::new_simple(
                        "unexpected break keyword in non-loop context",
                        state.clone(),
                    ));
                }
                Object::Continue => {
                    return Err(PanicObj::new_simple(
                        "unexpected continue keyword in non-loop context",
                        state.clone(),
                    ));
                }
                Object::ReturnVal(_ret_val) => return Ok(result.clone()),
                Object::Err(_) => return Ok(result.clone()),
                _ => {}
            }
        }

        Ok(result)
    }
}
