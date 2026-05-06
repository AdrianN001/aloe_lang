use crate::{
    frame::{
        expr_frame::{EvaluationResult, ExpressionFrame},
        state::ExpressionState,
    },
    object::{panic_obj::RuntimeSignal, stack_environment::EnvRef, state::StateRef},
};

impl ExpressionFrame {
    pub fn eval_step(
        &mut self,
        environ: EnvRef,
        interpreter_state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        match &mut self.state {
            ExpressionState::Await { future, state } => {
                state.eval_step(&self.expr, future, environ, interpreter_state)
            }
            ExpressionState::Primitive => match self.expr.evaluate(environ, interpreter_state) {
                Ok(ok_value) => Ok(EvaluationResult::Done(ok_value)),
                Err(e) => Err(e),
            },
        }
    }
}
