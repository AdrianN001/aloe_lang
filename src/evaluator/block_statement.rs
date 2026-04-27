use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::statement::block_statement::BlockStatement;
use crate::object::error::panic_type::PanicType;
use crate::object::panic_obj::{PanicObj, RuntimeSignal};
use crate::object::stack_environment::EnvRef;
use crate::object::state::StateRef;
use crate::object::{Object, ObjectRef};

impl BlockStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::ReturnVal(_ret_val) => {
                    if state.borrow().is_function_context() {
                        return Ok(result.clone());
                    } else {
                        return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                            PanicType::ReturnFromNonfunctionalContext,
                            "cannot return from a non-function context",
                            state.clone(),
                        )));
                    }
                }
                _ => {}
            }
        }

        Ok(result)
    }

    pub fn evaluate_with_function_context(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let mut result = Rc::new(RefCell::new(Object::NULL_OBJECT));

        for statement in self.statements.iter() {
            result = statement.evaluate(environ.clone(), state.clone())?;
            let borrowed_result = result.borrow();

            match &*borrowed_result {
                Object::BreakVal(_) => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected break keyword in non-loop context",
                        state.clone(),
                    )));
                }
                Object::Continue => {
                    return Err(RuntimeSignal::Panic(PanicObj::new_simple(
                        PanicType::UnexpectedKeyword,
                        "unexpected continue keyword in non-loop context",
                        state.clone(),
                    )));
                }
                Object::ReturnVal(_ret_val) => return Ok(result.clone()),
                _ => {}
            }
        }

        Ok(result)
    }
}
