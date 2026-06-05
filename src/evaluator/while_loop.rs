use crate::{
    ast::expression::{Expression, while_loop::WhileLoopExpression},
    object::{
        Object::{self},
        ObjectRef, new_objectref,
        panic_obj::RuntimeSignal,
        stack_environment::{EnvRef, StackEnvironment},
        state::StateRef,
    },
};

impl WhileLoopExpression {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }
        let new_environ = StackEnvironment::new_enclosed(
            environ,
            if let Some(condition) = &self.condition {
                format!("while {} {{...}}", condition.to_string())
            } else {
                "while {...}".to_string()
            },
        )
        .to_ref();

        {
            let mut env_borrow = new_environ.borrow_mut();
            env_borrow.set_loop_context(true);
        }

        match &self.condition {
            Some(_) => self.evaluate_loop_with_condition(new_environ, state),
            None => self.evaluate_loop_without_condition(new_environ, state),
        }
    }

    fn evaluate_loop_with_condition(
        &self,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<ObjectRef, RuntimeSignal> {
        let condition_expr = self.condition.clone().unwrap();

        let mut should_run =
            Self::check_if_expression_is_truthy(&condition_expr, environ.clone(), state.clone())?;

        while should_run {
            for statement in &self.block.statements {
                let _ = match statement.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Break(val)) => return Ok(val),
                    Err(RuntimeSignal::Continue) => break,
                    other_err => return other_err,
                };
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
    ) -> Result<ObjectRef, RuntimeSignal> {
        loop {
            for statement in &self.block.statements {
                let _ = match statement.evaluate(environ.clone(), state.clone()) {
                    Ok(result) => result,
                    Err(RuntimeSignal::Break(val)) => return Ok(val),
                    Err(RuntimeSignal::Continue) => break,
                    other_err => return other_err,
                };
            }
        }
    }

    fn check_if_expression_is_truthy(
        expr: &Expression,
        environ: EnvRef,
        state: StateRef,
    ) -> Result<bool, RuntimeSignal> {
        let object_created = expr.evaluate(environ, state)?;
        let borrowed = object_created.borrow();

        Ok(borrowed.is_truthy())
    }
}
