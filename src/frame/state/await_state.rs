use std::time::Instant;

use crate::{
    ast::expression::{Expression, await_expression::AwaitExpression},
    frame::expr_frame::{EvaluationResult, ExpressionFrame},
    object::{
        Object, ObjectRef,
        future::{future_kind::FutureKind, future_state::FutureState},
        new_objectref,
        panic_obj::RuntimeSignal,
        stack_environment::EnvRef,
        state::StateRef,
    },
};

#[derive(Debug, Clone)]
pub enum AwaitState {
    Start,
    Waiting,
    Done,
}

impl AwaitState {
    pub fn eval_step(
        &mut self,
        expression: &Expression,
        future_saved_in_frame: &mut Option<ObjectRef>,
        _environ: EnvRef,
        state: StateRef,
    ) -> Result<EvaluationResult, RuntimeSignal> {
        let _await_expr = {
            match expression {
                Expression::AwaitExpr(await_expr) => await_expr,
                other => panic!("{}", other.to_string()),
            }
        };

        {
            state
                .borrow_mut()
                .set_current_line(_await_expr.token.line_number);
        }

        match self {
            AwaitState::Start => {
                *self = AwaitState::Waiting;
                Ok(ExpressionFrame::build_frame_from_expr(
                    &_await_expr.expr,
                    _environ,
                ))
            }
            AwaitState::Waiting => {
                let future = future_saved_in_frame.as_ref().unwrap();

                let future_borrow = future.borrow();

                let future_raw = {
                    match &*future_borrow {
                        Object::Future(future_raw) => future_raw,
                        _ => {
                            // already awaited or doesnt need to be awaited at all
                            return Ok(EvaluationResult::Done(future.clone()));
                        }
                    }
                };
                match &future_raw.state {
                    FutureState::Ready(value) => {
                        let value = AwaitExpression::handle_return_value_according_the_expression(
                            _await_expr,
                            value.clone(),
                            state,
                        )?;
                        Ok(EvaluationResult::Done(value))
                    }
                    FutureState::Pending(kind) => match kind {
                        FutureKind::Sleep(sleep_till) => {
                            // we can just return null, since the future will be ready when the sleep is over, and it doesnt carry any value
                            let now = Instant::now();
                            if sleep_till <= &now {
                                Ok(EvaluationResult::Done(new_objectref(Object::NULL_OBJECT)))
                            } else {
                                Ok(EvaluationResult::Pending)
                            }
                        }
                        _ => Ok(EvaluationResult::Pending),
                    },
                    _ => panic!(),
                }
            }
            AwaitState::Done => unreachable!(),
        }
    }
}
