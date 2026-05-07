use crate::{
    ast::expression::{
        Expression, array_literal::ArrayLiteral, call_expression::CallExpression,
        index_expression::IndexExpression,
    },
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
                    state.current_argument += 1;
                    return Ok(EvaluationResult::Push(
                        call_expr.arguments[state.current_argument - 1].clone(),
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

                let function_name = {
                    let raw_name = match &*call_expr.function {
                        Expression::Identifier(identifier) => identifier.value.clone(),
                        _ => "anonymm function".to_string(),
                    };
                    raw_name
                };
                let return_value = CallExpression::call_with_arguments(
                    callable_object,
                    &args,
                    function_name,
                    interpreter_state,
                    environ,
                    questionmark_set,
                    bang_set,
                )?;
                Ok(EvaluationResult::Done(return_value))
            }

            ExpressionState::Array {
                ready_to_evaluate,
                state,
            } => {
                let number_of_elements = state.number_of_elements;

                let arr_expr = {
                    match &self.expr {
                        Expression::Array(arr) => arr,
                        _ => unreachable!(),
                    }
                };

                if !*ready_to_evaluate && number_of_elements != 0 {
                    state.curr_element += 1;
                    return Ok(EvaluationResult::Push(
                        arr_expr.elements[state.curr_element - 1].clone(),
                    ));
                }
                let elements = &state.elements;
                Ok(EvaluationResult::Done(ArrayLiteral::eval_step(
                    &elements,
                    interpreter_state.clone(),
                )?))
            }

            ExpressionState::Unary { value } => {
                let prefix_operator = {
                    let expr = &self.expr;
                    match expr {
                        Expression::Prefix(prefix) => &prefix.operator,
                        _ => unreachable!(),
                    }
                };

                let value_clone = value.as_ref().unwrap();
                let value_borrow = value_clone.borrow();
                Ok(EvaluationResult::Done(
                    value_borrow.evaluate_prefix(&prefix_operator, interpreter_state)?,
                ))
            }

            ExpressionState::Primitive => match self.expr.evaluate(environ, interpreter_state) {
                Ok(ok_value) => Ok(EvaluationResult::Done(ok_value)),
                Err(e) => Err(e),
            },
            ExpressionState::Index {
                ready_to_evaluate,
                state,
            } => {
                let index_expr = {
                    match &self.expr {
                        Expression::Index(indx) => indx,
                        _ => unreachable!(),
                    }
                };

                if !*ready_to_evaluate {
                    if state.indexable.is_none() {
                        return Ok(EvaluationResult::Push(*index_expr.left.clone()));
                    } else {
                        return Ok(EvaluationResult::Push(*index_expr.right.clone()));
                    }
                }

                let left = state.indexable.as_ref().unwrap();
                let right = state.index.as_ref().unwrap();
                Ok(EvaluationResult::Done(IndexExpression::eval_step(
                    left.clone(),
                    right.clone(),
                    interpreter_state.clone(),
                )?))
            }
        }
    }
}
