use crate::{
    ast::statement::expression_statement::ExpressionStatement,
    object::{ObjectRef, panic_obj::RuntimeSignal, stack_environment::EnvRef, state::StateRef},
};

impl ExpressionStatement {
    pub fn evaluate(&self, environ: EnvRef, state: StateRef) -> Result<ObjectRef, RuntimeSignal> {
        let expr = &self.expression;

        {
            state.borrow_mut().set_current_line(self.token.line_number);
        }

        match expr.evaluate(environ, state) {
            Ok(ok_value) => return Ok(ok_value),
            Err(RuntimeSignal::Yield(_)) => unreachable!(),
            Err(RuntimeSignal::Propagation(propagated_error)) => {
                Err(RuntimeSignal::Return(propagated_error.clone()))
            }
            other => return other,
        }
    }
}
