use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, while_loop::WhileLoopExpression},
        statement::Statement,
    },
    object::{
        Object, ObjectRef,
        error::panic_type::PanicType,
        new_objectref,
        panic_obj::PanicObj,
        stack_environment::{EnvRef, StackEnvironment},
        state::StateRef,
    },
};

impl WhileLoopExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, PanicObj> {
        let new_environ = Rc::new(RefCell::new(StackEnvironment::new_enclosed(
            environ,
            if let Some(condition) = &self.condition {
                format!("while {} {{...}}", condition.to_string())
            } else {
                "while {...}".to_string()
            },
        )));

        match &self.condition {
            Some(_) => self.evaluate_loop_with_condition(new_environ, state),
            None => self.evaluate_loop_without_condition(new_environ, state),
        }
    }

    fn evaluate_loop_with_condition(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        let condition_expr = self.condition.clone().unwrap();

        let mut should_run =
            Self::check_if_expression_is_truthy(&condition_expr, environ.clone(), state.clone())?;

        while should_run {
            for statement in &self.block.statements {
                if matches!(statement, Statement::Return(_))
                    && !state.borrow().is_function_context()
                {
                    return Err(PanicObj::new_simple(
                        PanicType::ReturnFromNonfunctionalContext,
                        "return statement was used in a non-function context",
                        state.clone(),
                    ));
                }
                let result = statement.evaluate(environ.clone(), state.clone())?;

                match &*result.borrow() {
                    Object::ReturnVal(_) => return Ok(result.clone()),
                    Object::BreakVal(break_val) => return Ok(*break_val.value.clone()),
                    Object::Continue => break,
                    Object::Err(_) => return Ok(result.clone()),
                    _ => {}
                }
            }

            should_run = Self::check_if_expression_is_truthy(
                &condition_expr,
                environ.clone(),
                state.clone(),
            )?;
        }

        Ok(new_objectref(Object::NULL_OBJECT))
    }

    fn evaluate_loop_without_condition(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, PanicObj> {
        loop {
            for statement in &self.block.statements {
                if matches!(statement, Statement::Return(_))
                    && !state.borrow().is_function_context()
                {
                    return Err(PanicObj::new_simple(
                        PanicType::ReturnFromNonfunctionalContext,
                        "return statement was used in a non-function context",
                        state.clone(),
                    ));
                }
                let result = statement.evaluate(environ.clone(), state.clone())?;

                match &*result.borrow() {
                    Object::ReturnVal(_) => return Ok(result.clone()),
                    Object::BreakVal(break_val) => return Ok(*break_val.value.clone()),
                    Object::Continue => break,
                    Object::Err(_) => return Ok(result.clone()),
                    _ => {}
                }
            }
        }
    }

    fn check_if_expression_is_truthy(
        expr: &Expression,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<bool, PanicObj> {
        let object_created = expr.evaluate(environ, state)?;
        let borrowed = object_created.borrow();

        Ok(borrowed.is_truthy())
    }
}
