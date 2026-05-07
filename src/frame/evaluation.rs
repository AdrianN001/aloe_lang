use crate::{
    ast::expression::{Expression, call_expression::CallExpression},
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
            ExpressionState::Call {
                state,
                ready_to_evaluate,
            } => {
                let call_expr = {
                    match &self.expr {
                        Expression::Call(call_expr) => call_expr,
                        _ => todo!(),
                    }
                };

                if !*ready_to_evaluate && state.parameters_required_by_func != 0 {
                    return Ok(EvaluationResult::Push(
                        call_expr.arguments[state.current_argument].clone(),
                    ));
                }
                let (callable_object, questionmark_set, bang_set) = {
                    let callable_object = call_expr
                        .function
                        .evaluate(environ.clone(), interpreter_state.clone())?;
                    let questionmark_set = call_expr.question_mark_set;
                    let bang_set = call_expr.bang_set;

                    (callable_object, questionmark_set, bang_set)
                };
                let args = &state.parameters;
                let function_name = "async function".into(); //TODO
                let return_value = CallExpression::call_with_arguments(
                    callable_object,
                    &args,
                    function_name,
                    interpreter_state,
                    environ,
                    questionmark_set,
                    bang_set,
                )?;
                {
                    let ret_val_borrow = return_value.borrow();
                    println!("{}", ret_val_borrow.inspect());
                }
                Ok(EvaluationResult::Done(return_value))
            }

            ExpressionState::Primitive => match self.expr.evaluate(environ, interpreter_state) {
                Ok(ok_value) => Ok(EvaluationResult::Done(ok_value)),
                Err(e) => Err(e),
            },
        }
    }
}
